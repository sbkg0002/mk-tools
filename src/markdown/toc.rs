use crate::domain::codeblock::TextSpan;
use crate::domain::toc::{make_anchors_unique, Heading, TocOptions, TocRegionSpec, TocStyle};
use anyhow::{Context, Result};
use regex::Regex;
use std::path::PathBuf;

/// Find all TOC regions in markdown content
pub fn find_toc_regions(content: &str) -> Result<Vec<TocRegionSpec>> {
    let start_regex = Regex::new(r"<!--\s*mk-toc:start([^>]*?)-->").unwrap();
    let end_regex = Regex::new(r"<!--\s*mk-toc:end\s*-->").unwrap();

    let mut regions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut byte_offset = 0;

    let mut pending_start: Option<(TextSpan, usize, String, TocOptions)> = None;

    for (line_idx, line) in lines.iter().enumerate() {
        let line_number = line_idx + 1;
        let line_start = byte_offset;
        let _line_end = byte_offset + line.len();

        // Check for start marker
        if let Some(captures) = start_regex.captures(line) {
            if pending_start.is_some() {
                anyhow::bail!(
                    "Found mk-toc:start at line {} without matching mk-toc:end for previous start",
                    line_number
                );
            }

            let full_match = captures.get(0).unwrap();
            let marker_start = line_start + full_match.start();
            let marker_end = line_start + full_match.end();

            let options_str = captures.get(1).map(|m| m.as_str()).unwrap_or("");
            let options = parse_toc_options(options_str)?;

            pending_start = Some((
                TextSpan::new(marker_start, marker_end),
                line_number,
                line.to_string(),
                options,
            ));
        }

        // Check for end marker
        if let Some(captures) = end_regex.captures(line) {
            if let Some((start_span, start_line, raw_marker, options)) = pending_start.take() {
                let full_match = captures.get(0).unwrap();
                let marker_start = line_start + full_match.start();
                let marker_end = line_start + full_match.end();

                let region = TocRegionSpec::new(
                    start_span,
                    TextSpan::new(marker_start, marker_end),
                    start_line,
                    line_number,
                    raw_marker,
                )
                .with_options(options);

                regions.push(region);
            } else {
                anyhow::bail!(
                    "Found mk-toc:end at line {} without matching mk-toc:start",
                    line_number
                );
            }
        }

        byte_offset += line.len() + 1; // +1 for newline
    }

    if pending_start.is_some() {
        anyhow::bail!("Found mk-toc:start without matching mk-toc:end");
    }

    Ok(regions)
}

/// Parse TOC options from the start marker
fn parse_toc_options(options_str: &str) -> Result<TocOptions> {
    let mut options = TocOptions::default();

    // Match key=value pairs
    let option_regex = Regex::new(r"(\w+(?:-\w+)*)=([^\s]+)").unwrap();

    for captures in option_regex.captures_iter(options_str) {
        let key = captures.get(1).unwrap().as_str();
        let value = captures.get(2).unwrap().as_str();

        match key {
            "from-level" => {
                options.from_level = value
                    .parse()
                    .with_context(|| format!("Invalid from-level value: {}", value))?;
            }
            "to-level" => {
                options.to_level = value
                    .parse()
                    .with_context(|| format!("Invalid to-level value: {}", value))?;
            }
            "style" => {
                options.style = TocStyle::from_str(value)
                    .ok_or_else(|| anyhow::anyhow!("Invalid style value: {}", value))?;
            }
            "root" => {
                options.root = Some(PathBuf::from(value));
            }
            _ => {
                log::warn!("Unknown TOC option: {}", key);
            }
        }
    }

    Ok(options)
}

/// Extract all headings from markdown content
pub fn extract_headings(content: &str) -> Vec<Heading> {
    let heading_regex = Regex::new(r"^(#{1,6})\s+(.+?)(?:\s*#*\s*)?$").unwrap();
    let mut headings = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        let line_number = line_idx + 1;

        if let Some(captures) = heading_regex.captures(line) {
            let hashes = captures.get(1).unwrap().as_str();
            let level = hashes.len() as u8;
            let text = captures.get(2).unwrap().as_str().trim().to_string();

            let heading = Heading::new(level, text, line_number);
            headings.push(heading);
        }
    }

    // Make anchors unique
    make_anchors_unique(&mut headings);

    headings
}

/// Generate TOC content from headings
pub fn generate_toc(headings: &[Heading], options: &TocOptions) -> String {
    // Filter headings by level
    let filtered: Vec<&Heading> = headings
        .iter()
        .filter(|h| options.includes_level(h.level))
        .collect();

    if filtered.is_empty() {
        return String::new();
    }

    let mut lines = Vec::new();

    // Find the minimum level to use as base indentation
    let min_level = filtered.iter().map(|h| h.level).min().unwrap_or(1);

    for heading in filtered {
        let indent_level = (heading.level - min_level) as usize;
        let indent = "  ".repeat(indent_level);

        let list_marker = match options.style {
            TocStyle::Bullet => "-",
            TocStyle::Numbered => "1.",
        };

        let line = format!("{}{} {}", indent, list_marker, heading.to_link());
        lines.push(line);
    }

    lines.join("\n")
}

/// Apply TOC updates to markdown content
pub fn apply_toc_updates(
    original_content: &str,
    regions: &[TocRegionSpec],
    headings: &[Heading],
) -> Result<String> {
    let mut result = original_content.to_string();

    // Process regions in reverse order to maintain byte offsets
    let mut sorted_regions: Vec<&TocRegionSpec> = regions.iter().collect();
    sorted_regions.sort_by_key(|r| std::cmp::Reverse(r.start_span.start));

    for region in sorted_regions {
        // Generate TOC for this region
        let toc_content = generate_toc(headings, &region.options);

        // Get the span to replace (content between start and end markers)
        let content_span = region.content_span();

        // Build the replacement text with proper newlines
        let replacement = if toc_content.is_empty() {
            String::from("\n")
        } else {
            format!("\n{}\n", toc_content)
        };

        // Replace the content
        result.replace_range(content_span.start..content_span.end, &replacement);
    }

    Ok(result)
}

/// Check if TOC regions are up to date
pub fn check_toc_up_to_date(original_content: &str, regions: &[TocRegionSpec]) -> Result<bool> {
    let headings = extract_headings(original_content);
    let updated_content = apply_toc_updates(original_content, regions, &headings)?;
    Ok(original_content == updated_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toc_options_default() {
        let opts = parse_toc_options("").unwrap();
        assert_eq!(opts.from_level, 2);
        assert_eq!(opts.to_level, 6);
        assert_eq!(opts.style, TocStyle::Bullet);
    }

    #[test]
    fn test_parse_toc_options_custom() {
        let opts = parse_toc_options(" from-level=1 to-level=3 style=numbered").unwrap();
        assert_eq!(opts.from_level, 1);
        assert_eq!(opts.to_level, 3);
        assert_eq!(opts.style, TocStyle::Numbered);
    }

    #[test]
    fn test_extract_headings() {
        let content = r#"# Title
## Introduction
Some text here.
### Details
More text.
## Conclusion
"#;

        let headings = extract_headings(content);
        assert_eq!(headings.len(), 4);
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[0].text, "Title");
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[1].text, "Introduction");
        assert_eq!(headings[2].level, 3);
        assert_eq!(headings[2].text, "Details");
    }

    #[test]
    fn test_extract_headings_with_trailing_hashes() {
        let content = "## Heading ##\n### Another Heading ###";
        let headings = extract_headings(content);
        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].text, "Heading");
        assert_eq!(headings[1].text, "Another Heading");
    }

    #[test]
    fn test_generate_toc_bullet() {
        let headings = vec![
            Heading::new(2, "Introduction".to_string(), 1),
            Heading::new(3, "Getting Started".to_string(), 5),
            Heading::new(2, "Usage".to_string(), 10),
        ];

        let options = TocOptions::default();
        let toc = generate_toc(&headings, &options);

        assert!(toc.contains("- [Introduction](#introduction)"));
        assert!(toc.contains("  - [Getting Started](#getting-started)"));
        assert!(toc.contains("- [Usage](#usage)"));
    }

    #[test]
    fn test_generate_toc_numbered() {
        let headings = vec![
            Heading::new(2, "First".to_string(), 1),
            Heading::new(2, "Second".to_string(), 5),
        ];

        let options = TocOptions::default().with_style(TocStyle::Numbered);
        let toc = generate_toc(&headings, &options);

        assert!(toc.contains("1. [First](#first)"));
        assert!(toc.contains("1. [Second](#second)"));
    }

    #[test]
    fn test_generate_toc_filters_levels() {
        let headings = vec![
            Heading::new(1, "Title".to_string(), 1),
            Heading::new(2, "Introduction".to_string(), 2),
            Heading::new(3, "Details".to_string(), 3),
            Heading::new(4, "More Details".to_string(), 4),
        ];

        let options = TocOptions::default().with_from_level(2).with_to_level(3);
        let toc = generate_toc(&headings, &options);

        assert!(!toc.contains("Title"));
        assert!(toc.contains("Introduction"));
        assert!(toc.contains("Details"));
        assert!(!toc.contains("More Details"));
    }

    #[test]
    fn test_find_toc_regions() {
        let content = r#"# Document

<!-- mk-toc:start -->
<!-- mk-toc:end -->

## Section 1

<!-- mk-toc:start from-level=3 -->
<!-- mk-toc:end -->
"#;

        let regions = find_toc_regions(content).unwrap();
        assert_eq!(regions.len(), 2);
        assert_eq!(regions[0].options.from_level, 2); // default
        assert_eq!(regions[1].options.from_level, 3); // custom
    }

    #[test]
    fn test_find_toc_regions_unpaired_start() {
        let content = "<!-- mk-toc:start -->\nSome content";
        let result = find_toc_regions(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_toc_regions_unpaired_end() {
        let content = "Some content\n<!-- mk-toc:end -->";
        let result = find_toc_regions(content);
        assert!(result.is_err());
    }
}
