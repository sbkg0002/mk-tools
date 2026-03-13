use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use std::process;

mod cli;
mod domain;
mod fs;
mod logging;
mod markdown;

use cli::{CheckArgs, Cli, CodeblocksArgs, Commands, TocArgs};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    use clap::Parser;
    let cli = Cli::parse();

    // Initialize logging
    logging::init_logging(cli.quiet, cli.verbose, cli.color);

    // Change directory if requested
    if let Some(ref chdir) = cli.chdir {
        env::set_current_dir(chdir)
            .with_context(|| format!("Failed to change directory to: {}", chdir.display()))?;
        log::debug!("Changed directory to: {}", chdir.display());
    }

    // Dispatch to command handlers
    match cli.command {
        Commands::Codeblocks(args) => handle_codeblocks(args, cli.dry_run),
        Commands::Toc(args) => handle_toc(args, cli.dry_run),
        Commands::Check(args) => handle_check(args),
        Commands::Version => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

fn handle_codeblocks(args: CodeblocksArgs, dry_run: bool) -> Result<()> {
    log::info!("Running codeblocks command");

    // Validate encoding
    if args.encoding.to_lowercase() != "utf-8" {
        anyhow::bail!("Only UTF-8 encoding is currently supported");
    }

    // Discover markdown files (default to current directory if no paths provided)
    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };
    let markdown_files = fs::discover_markdown_files(&paths, &args.glob)?;

    if markdown_files.is_empty() {
        log::warn!("No Markdown files found");
        return Ok(());
    }

    log::info!("Processing {} Markdown file(s)", markdown_files.len());

    let mut files_changed = 0;
    let mut files_with_errors = 0;

    for file_path in &markdown_files {
        log::debug!("Processing: {}", file_path.display());

        match process_codeblocks_file(file_path, &args, dry_run) {
            Ok(changed) => {
                if changed {
                    files_changed += 1;
                    if args.check {
                        log::warn!("File needs update: {}", file_path.display());
                    } else if dry_run {
                        log::info!("Would update: {}", file_path.display());
                    } else {
                        log::info!("Updated: {}", file_path.display());
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to process {}: {:#}", file_path.display(), e);
                files_with_errors += 1;
            }
        }
    }

    log::info!(
        "Summary: {} file(s) changed, {} file(s) with errors",
        files_changed,
        files_with_errors
    );

    if files_with_errors > 0 {
        anyhow::bail!("{} file(s) had errors", files_with_errors);
    }

    if args.check && files_changed > 0 {
        anyhow::bail!("{} file(s) need updates", files_changed);
    }

    Ok(())
}

fn process_codeblocks_file(file_path: &Path, args: &CodeblocksArgs, dry_run: bool) -> Result<bool> {
    // Read the file
    let content = fs::read_file(file_path)?;

    // Find codeblock markers
    let language_overrides = args.language_overrides.as_deref().unwrap_or(&[]);
    let specs = markdown::find_codeblock_markers(
        &content,
        file_path,
        args.root.as_deref(),
        language_overrides,
    )?;

    if specs.is_empty() {
        log::debug!("No codeblock markers found in {}", file_path.display());
        return Ok(false);
    }

    log::debug!("Found {} codeblock marker(s)", specs.len());

    // Apply updates
    let updated_content = markdown::apply_codeblock_updates(&content, &specs)?;

    // Check if content changed
    let changed = content != updated_content;

    // Write back if changed and not in check/dry-run mode
    if changed && !args.check && !dry_run {
        fs::write_file_with_backup(file_path, &updated_content, !args.no_backup)?;
    }

    Ok(changed)
}

fn handle_toc(args: TocArgs, dry_run: bool) -> Result<()> {
    log::info!("Running toc command");

    // Validate encoding
    if args.encoding.to_lowercase() != "utf-8" {
        anyhow::bail!("Only UTF-8 encoding is currently supported");
    }

    // Check if cross-file TOC is requested
    if let Some(from_dir) = args.from_dir.clone() {
        return handle_cross_file_toc(args, &from_dir, dry_run);
    }

    // Discover markdown files (default to current directory if no paths provided)
    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };
    let markdown_files = fs::discover_markdown_files(&paths, &args.glob)?;

    if markdown_files.is_empty() {
        log::warn!("No Markdown files found");
        return Ok(());
    }

    log::info!("Processing {} Markdown file(s)", markdown_files.len());

    let mut files_changed = 0;
    let mut files_with_errors = 0;

    for file_path in &markdown_files {
        log::debug!("Processing: {}", file_path.display());

        match process_toc_file(file_path, &args, dry_run) {
            Ok(changed) => {
                if changed {
                    files_changed += 1;
                    if args.check {
                        log::warn!("File needs update: {}", file_path.display());
                    } else if dry_run {
                        log::info!("Would update: {}", file_path.display());
                    } else {
                        log::info!("Updated: {}", file_path.display());
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to process {}: {:#}", file_path.display(), e);
                files_with_errors += 1;
            }
        }
    }

    log::info!(
        "Summary: {} file(s) changed, {} file(s) with errors",
        files_changed,
        files_with_errors
    );

    if files_with_errors > 0 {
        anyhow::bail!("{} file(s) had errors", files_with_errors);
    }

    if args.check && files_changed > 0 {
        anyhow::bail!("{} file(s) need updates", files_changed);
    }

    Ok(())
}

fn handle_cross_file_toc(args: TocArgs, from_dir: &PathBuf, dry_run: bool) -> Result<()> {
    log::info!(
        "Running cross-file toc command from directory: {}",
        from_dir.display()
    );

    // Discover source files from from_dir
    let source_files = fs::discover_markdown_files(std::slice::from_ref(from_dir), &args.glob)?;

    if source_files.is_empty() {
        log::warn!("No source Markdown files found in: {}", from_dir.display());
        return Ok(());
    }

    log::info!(
        "Found {} source file(s) in {}",
        source_files.len(),
        from_dir.display()
    );

    // Extract headings from all source files
    let headings = markdown::toc::extract_headings_from_files(&source_files)
        .with_context(|| format!("Failed to extract headings from {}", from_dir.display()))?;

    log::debug!(
        "Extracted {} total headings from source files",
        headings.len()
    );

    // Determine target files (files to update with cross-file TOC)
    let target_paths = if args.paths.is_empty() {
        vec![PathBuf::from("README.md")] // Default to README.md
    } else {
        args.paths.clone()
    };

    let target_files = fs::discover_markdown_files(&target_paths, &args.glob)?;

    if target_files.is_empty() {
        log::warn!("No target Markdown files found");
        return Ok(());
    }

    log::info!("Processing {} target file(s)", target_files.len());

    let mut files_changed = 0;
    let mut files_with_errors = 0;

    for file_path in &target_files {
        log::debug!("Processing target: {}", file_path.display());

        match process_cross_file_toc(file_path, &headings, from_dir, &args, dry_run) {
            Ok(changed) => {
                if changed {
                    files_changed += 1;
                    if args.check {
                        log::warn!("File needs update: {}", file_path.display());
                    } else if dry_run {
                        log::info!("Would update: {}", file_path.display());
                    } else {
                        log::info!("Updated: {}", file_path.display());
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to process {}: {:#}", file_path.display(), e);
                files_with_errors += 1;
            }
        }
    }

    log::info!(
        "Summary: {} file(s) changed, {} file(s) with errors",
        files_changed,
        files_with_errors
    );

    if files_with_errors > 0 {
        anyhow::bail!("{} file(s) had errors", files_with_errors);
    }

    if args.check && files_changed > 0 {
        anyhow::bail!("{} file(s) need updates", files_changed);
    }

    Ok(())
}

fn process_toc_file(file_path: &Path, args: &TocArgs, dry_run: bool) -> Result<bool> {
    // Read the file
    let original_content = fs::read_file(file_path)?;
    let mut content = original_content.clone();

    // Find TOC regions
    let mut regions = markdown::find_toc_regions(&content)?;

    // If no regions found and --add is specified, insert markers
    if regions.is_empty() && args.add {
        log::info!("Adding TOC markers to {}", file_path.display());
        content = markdown::insert_toc_markers_after_h1(&content)?;
        regions = markdown::find_toc_regions(&content)?;
    }

    if regions.is_empty() {
        log::debug!("No TOC regions found in {}", file_path.display());
        return Ok(false);
    }

    log::debug!("Found {} TOC region(s)", regions.len());

    // Override from_level if --include-h1 is set
    if args.include_h1 {
        for region in &mut regions {
            region.options.from_level = 1;
        }
    }

    // Extract headings
    let headings = markdown::extract_headings(&content);

    // Apply updates
    let updated_content = markdown::apply_toc_updates(&content, &regions, &headings)?;

    // Check if content changed (compare against original content)
    let changed = original_content != updated_content;

    // Write back if changed and not in check/dry-run mode
    if changed && !args.check && !dry_run {
        fs::write_file_with_backup(file_path, &updated_content, !args.no_backup)?;
    }

    Ok(changed)
}

fn process_cross_file_toc(
    file_path: &Path,
    headings: &[markdown::toc::HeadingWithFile],
    _base_dir: &Path,
    args: &TocArgs,
    dry_run: bool,
) -> Result<bool> {
    use crate::domain::toc::TocRegionSpec;
    use crate::markdown::toc::generate_cross_file_toc;

    // Read the file
    let content = fs::read_file(file_path)?;

    // Find TOC regions
    let mut regions = markdown::find_toc_regions(&content)?;

    if regions.is_empty() {
        log::debug!("No TOC regions found in {}", file_path.display());
        return Ok(false);
    }

    log::debug!("Found {} TOC region(s)", regions.len());

    // Override from_level if --include-h1 is set
    if args.include_h1 {
        for region in &mut regions {
            region.options.from_level = 1;
        }
    }

    // Generate cross-file TOC
    let mut result = content.clone();

    // Process regions in reverse order to maintain byte offsets
    let mut sorted_regions: Vec<&TocRegionSpec> = regions.iter().collect();
    sorted_regions.sort_by_key(|r| std::cmp::Reverse(r.start_span.start));

    for region in sorted_regions {
        // Use target file's parent directory as base for relative links
        let target_base = file_path.parent().unwrap_or_else(|| Path::new("."));
        let toc_content = generate_cross_file_toc(headings, &region.options, Some(target_base));

        let content_span = region.content_span();
        let replacement = if toc_content.is_empty() {
            String::from("\n")
        } else {
            format!("\n{}\n", toc_content)
        };

        result.replace_range(content_span.start..content_span.end, &replacement);
    }

    // Check if content changed
    let changed = content != result;

    // Write back if changed and not in check/dry-run mode
    if changed && !args.check && !dry_run {
        fs::write_file_with_backup(file_path, &result, !args.no_backup)?;
    }

    Ok(changed)
}

fn handle_check(args: CheckArgs) -> Result<()> {
    log::info!("Running check command");

    // Discover markdown files (default to current directory if no paths provided)
    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };
    let markdown_files = fs::discover_markdown_files(&paths, &args.glob)?;

    if markdown_files.is_empty() {
        log::warn!("No Markdown files found");
        return Ok(());
    }

    log::info!("Checking {} Markdown file(s)", markdown_files.len());

    let mut files_needing_update = 0;
    let mut files_with_errors = 0;

    for file_path in &markdown_files {
        log::debug!("Checking: {}", file_path.display());

        match check_file(file_path, &args) {
            Ok(needs_update) => {
                if needs_update {
                    files_needing_update += 1;
                    log::warn!("File needs update: {}", file_path.display());
                }
            }
            Err(e) => {
                log::error!("Failed to check {}: {:#}", file_path.display(), e);
                files_with_errors += 1;
            }
        }
    }

    log::info!(
        "Summary: {} file(s) need updates, {} file(s) with errors",
        files_needing_update,
        files_with_errors
    );

    if files_with_errors > 0 {
        anyhow::bail!("{} file(s) had errors", files_with_errors);
    }

    if files_needing_update > 0 {
        anyhow::bail!("{} file(s) need updates", files_needing_update);
    }

    Ok(())
}

fn check_file(file_path: &Path, args: &CheckArgs) -> Result<bool> {
    let content = fs::read_file(file_path)?;

    // Check codeblocks
    let codeblock_specs =
        markdown::find_codeblock_markers(&content, file_path, args.root.as_deref(), &[])?;
    let codeblocks_ok = if !codeblock_specs.is_empty() {
        markdown::check_codeblocks_up_to_date(&content, &codeblock_specs)?
    } else {
        true
    };

    // Check TOC
    let toc_regions = markdown::find_toc_regions(&content)?;
    let toc_ok = if !toc_regions.is_empty() {
        markdown::check_toc_up_to_date(&content, &toc_regions)?
    } else {
        true
    };

    Ok(!codeblocks_ok || !toc_ok)
}
