use crate::domain::codeblock::{CodeblockOptions, CodeblockSpec, LineRange, TextSpan};
use crate::fs::path_utils::{
    build_language_overrides, extension_to_language, get_extension, resolve_path,
};
use anyhow::{Context, Result};
use regex::Regex;

use std::path::{Path, PathBuf};

/// Parse markdown content and find all codeblock markers
pub fn find_codeblock_markers(
    content: &str,
    markdown_file_path: &Path,
    root: Option<&Path>,
    language_overrides: &[(String, String)],
) -> Result<Vec<CodeblockSpec>> {
    let marker_regex = Regex::new(r"<!--\s*mk-code:\s*([^\s]+)([^>]*?)-->").unwrap();

    let lang_map = build_language_overrides(language_overrides);
    let mut specs = Vec::new();

    let base_dir = root.map(|p| p.to_path_buf()).unwrap_or_else(|| {
        markdown_file_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    });

    let lines: Vec<&str> = content.lines().collect();
    let mut byte_offset = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let line_number = line_idx + 1;
        let line_start = byte_offset;
        let _line_end = byte_offset + line.len();

        if let Some(captures) = marker_regex.captures(line) {
            let full_match = captures.get(0).unwrap();
            let marker_start = line_start + full_match.start();
            let marker_end = line_start + full_match.end();

            let path_str = captures.get(1).unwrap().as_str();
            let options_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");

            // Parse options
            let options = parse_codeblock_options(options_str)?;

            // Resolve source file path
            let source_path = resolve_path(Path::new(path_str), &base_dir);

            // Determine language
            let lang = if let Some(ref explicit_lang) = options.lang {
                Some(explicit_lang.clone())
            } else if let Some(ext) = get_extension(&source_path) {
                extension_to_language(&ext, &lang_map)
            } else {
                None
            };

            // Look for an existing code block on the next non-empty line
            let existing_block_span =
                find_code_block_after_marker(line_idx + 1, &lines, byte_offset + line.len() + 1);

            let spec = CodeblockSpec::new(
                TextSpan::new(marker_start, marker_end),
                line_number,
                source_path,
                line.to_string(),
            )
            .with_lang(lang)
            .with_line_range(options.line_range())
            .with_dedent(options.dedent)
            .with_existing_block(existing_block_span);

            specs.push(spec);
        }

        byte_offset += line.len() + 1; // +1 for newline
    }

    Ok(specs)
}

/// Parse options from the marker comment
fn parse_codeblock_options(options_str: &str) -> Result<CodeblockOptions> {
    let mut options = CodeblockOptions::new();

    // Match key=value pairs
    let option_regex = Regex::new(r"(\w+)=([^\s]+)").unwrap();

    for captures in option_regex.captures_iter(options_str) {
        let key = captures.get(1).unwrap().as_str();
        let value = captures.get(2).unwrap().as_str();

        match key {
            "lang" => options.lang = Some(value.to_string()),
            "start" => {
                options.start = Some(
                    value
                        .parse()
                        .with_context(|| format!("Invalid start line number: {}", value))?,
                );
            }
            "end" => {
                options.end = Some(
                    value
                        .parse()
                        .with_context(|| format!("Invalid end line number: {}", value))?,
                );
            }
            "dedent" => {
                options.dedent = Some(
                    value
                        .parse()
                        .with_context(|| format!("Invalid dedent value: {}", value))?,
                );
            }
            "region" => {
                options.region = Some(value.to_string());
            }
            _ => {
                log::warn!("Unknown codeblock option: {}", key);
            }
        }
    }

    Ok(options)
}

/// Find a code block starting after a given line
fn find_code_block_after_marker(
    start_line_idx: usize,
    lines: &[&str],
    mut byte_offset: usize,
) -> Option<TextSpan> {
    let fence_regex = Regex::new(r"^```(\w*)").unwrap();

    // Skip empty lines and find the fence
    for (idx, line) in lines.iter().enumerate().skip(start_line_idx) {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            byte_offset += line.len() + 1;
            continue;
        }

        if fence_regex.is_match(trimmed) {
            // Found opening fence
            let block_start = byte_offset;
            byte_offset += line.len() + 1;

            // Find closing fence
            for line in lines.iter().skip(idx + 1) {
                if line.trim().starts_with("```") {
                    // Found closing fence
                    let block_end = byte_offset + line.len();
                    return Some(TextSpan::new(block_start, block_end));
                }
                byte_offset += line.len() + 1;
            }

            // No closing fence found - treat as end of file
            return Some(TextSpan::new(block_start, byte_offset));
        }

        // Non-empty, non-fence line means no code block immediately follows
        break;
    }

    None
}

/// Read and process source file content
pub fn read_source_content(
    source_path: &Path,
    line_range: Option<LineRange>,
    dedent: Option<usize>,
) -> Result<String> {
    let content = crate::fs::read_file(source_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Apply line range if specified
    let selected_lines: Vec<&str> = if let Some(range) = line_range {
        lines
            .iter()
            .enumerate()
            .filter(|(idx, _)| {
                let line_num = idx + 1;
                range.contains(line_num)
            })
            .map(|(_, line)| *line)
            .collect()
    } else {
        lines
    };

    // Apply dedenting if specified
    let processed_lines: Vec<String> = if let Some(dedent_amount) = dedent {
        selected_lines
            .iter()
            .map(|line| dedent_line(line, dedent_amount))
            .collect()
    } else {
        selected_lines.iter().map(|s| s.to_string()).collect()
    };

    Ok(processed_lines.join("\n"))
}

/// Remove leading spaces from a line
fn dedent_line(line: &str, amount: usize) -> String {
    let spaces_to_remove = line.chars().take(amount).take_while(|c| *c == ' ').count();

    line.chars().skip(spaces_to_remove).collect()
}

/// Generate a code fence block
pub fn generate_code_block(content: &str, lang: Option<&str>) -> String {
    let lang_str = lang.unwrap_or("");
    format!("```{}\n{}\n```", lang_str, content)
}

/// Apply codeblock updates to markdown content
pub fn apply_codeblock_updates(original_content: &str, specs: &[CodeblockSpec]) -> Result<String> {
    let mut result = original_content.to_string();
    // Process specs in order (we'll track offset changes)
    // For simplicity, we process from end to start to avoid offset issues
    let mut sorted_specs: Vec<&CodeblockSpec> = specs.iter().collect();
    sorted_specs.sort_by_key(|s| std::cmp::Reverse(s.marker_span.start));

    for spec in sorted_specs {
        // Read the source file
        let source_content = read_source_content(&spec.source_path, spec.line_range, spec.dedent)
            .with_context(|| {
            format!(
                "Failed to read source file '{}' referenced at line {}",
                spec.source_path.display(),
                spec.marker_line
            )
        })?;

        // Generate the new code block
        let new_block = generate_code_block(&source_content, spec.lang.as_deref());

        // Determine where to insert/replace
        if let Some(existing_span) = spec.existing_block_span {
            // Replace existing block
            let start = existing_span.start;
            let end = existing_span.end;
            result.replace_range(start..end, &new_block);
        } else {
            // Insert new block after marker
            // Find the end of the marker line
            let insert_pos = spec.marker_span.end;

            // Find the next newline and insert after it
            if let Some(newline_pos) = result[insert_pos..].find('\n') {
                let insert_at = insert_pos + newline_pos + 1;
                result.insert_str(insert_at, &format!("{}\n", new_block));
            } else {
                // No newline after marker, append at end
                result.push_str(&format!("\n{}\n", new_block));
            }
        }
    }

    Ok(result)
}

/// Check if codeblocks are up to date
pub fn check_codeblocks_up_to_date(
    original_content: &str,
    specs: &[CodeblockSpec],
) -> Result<bool> {
    let updated_content = apply_codeblock_updates(original_content, specs)?;
    Ok(original_content == updated_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_codeblock_options() {
        let opts = parse_codeblock_options(" lang=rust start=10 end=20 dedent=4").unwrap();
        assert_eq!(opts.lang, Some("rust".to_string()));
        assert_eq!(opts.start, Some(10));
        assert_eq!(opts.end, Some(20));
        assert_eq!(opts.dedent, Some(4));
    }

    #[test]
    fn test_dedent_line() {
        assert_eq!(dedent_line("    hello", 4), "hello");
        assert_eq!(dedent_line("  hello", 4), "hello");
        assert_eq!(dedent_line("hello", 4), "hello");
        assert_eq!(dedent_line("    hello", 2), "  hello");
    }

    #[test]
    fn test_generate_code_block() {
        let block = generate_code_block("fn main() {}", Some("rust"));
        assert_eq!(block, "```rust\nfn main() {}\n```");

        let block_no_lang = generate_code_block("content", None);
        assert_eq!(block_no_lang, "```\ncontent\n```");
    }

    #[test]
    fn test_read_source_content_with_range() {
        let dir = tempdir().unwrap();
        let source_path = dir.path().join("test.txt");
        fs::write(&source_path, "line1\nline2\nline3\nline4\nline5").unwrap();

        let content = read_source_content(&source_path, Some(LineRange::new(2, 4)), None).unwrap();

        assert_eq!(content, "line2\nline3\nline4");
    }

    #[test]
    fn test_read_source_content_with_dedent() {
        let dir = tempdir().unwrap();
        let source_path = dir.path().join("test.txt");
        fs::write(&source_path, "    line1\n    line2\n    line3").unwrap();

        let content = read_source_content(&source_path, None, Some(4)).unwrap();

        assert_eq!(content, "line1\nline2\nline3");
    }
}
