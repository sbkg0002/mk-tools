use anyhow::{Context, Result};
use std::env;
use std::path::Path;
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

    // Discover markdown files
    let markdown_files = fs::discover_markdown_files(&args.paths, &args.glob)?;

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

    // Discover markdown files
    let markdown_files = fs::discover_markdown_files(&args.paths, &args.glob)?;

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

fn process_toc_file(file_path: &Path, args: &TocArgs, dry_run: bool) -> Result<bool> {
    // Read the file
    let content = fs::read_file(file_path)?;

    // Find TOC regions
    let regions = markdown::find_toc_regions(&content)?;

    if regions.is_empty() {
        log::debug!("No TOC regions found in {}", file_path.display());
        return Ok(false);
    }

    log::debug!("Found {} TOC region(s)", regions.len());

    // Extract headings
    let headings = markdown::extract_headings(&content);

    // Apply updates
    let updated_content = markdown::apply_toc_updates(&content, &regions, &headings)?;

    // Check if content changed
    let changed = content != updated_content;

    // Write back if changed and not in check/dry-run mode
    if changed && !args.check && !dry_run {
        fs::write_file_with_backup(file_path, &updated_content, !args.no_backup)?;
    }

    Ok(changed)
}

fn handle_check(args: CheckArgs) -> Result<()> {
    log::info!("Running check command");

    // Discover markdown files
    let markdown_files = fs::discover_markdown_files(&args.paths, &args.glob)?;

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
