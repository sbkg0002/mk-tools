use crate::domain::codeblock::TextSpan;
use std::path::PathBuf;

/// Style of the table of contents list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TocStyle {
    /// Bullet list style (using `-`)
    #[default]
    Bullet,
    /// Numbered list style (using `1.`)
    Numbered,
}

impl TocStyle {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bullet" => Some(Self::Bullet),
            "numbered" => Some(Self::Numbered),
            _ => None,
        }
    }
}

/// Options for generating a table of contents
#[derive(Debug, Clone)]
pub struct TocOptions {
    /// Minimum heading level to include (default: 2)
    pub from_level: u8,

    /// Maximum heading level to include (default: 6)
    pub to_level: u8,

    /// List style for the TOC
    pub style: TocStyle,

    /// Optional base path for anchor links
    pub root: Option<PathBuf>,
}

impl Default for TocOptions {
    fn default() -> Self {
        Self {
            from_level: 2,
            to_level: 6,
            style: TocStyle::default(),
            root: None,
        }
    }
}

impl TocOptions {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn with_from_level(mut self, level: u8) -> Self {
        self.from_level = level;
        self
    }

    #[allow(dead_code)]
    pub fn with_to_level(mut self, level: u8) -> Self {
        self.to_level = level;
        self
    }

    #[allow(dead_code)]
    pub fn with_style(mut self, style: TocStyle) -> Self {
        self.style = style;
        self
    }

    #[allow(dead_code)]
    pub fn with_root(mut self, root: Option<PathBuf>) -> Self {
        self.root = root;
        self
    }

    /// Check if a heading level should be included in the TOC
    pub fn includes_level(&self, level: u8) -> bool {
        level >= self.from_level && level <= self.to_level
    }
}

/// Specification for a TOC region in a Markdown file
#[derive(Debug, Clone)]
pub struct TocRegionSpec {
    /// Span of the mk-toc:start comment
    pub start_span: TextSpan,

    /// Span of the mk-toc:end comment
    pub end_span: TextSpan,

    /// Line number where the start marker appears (1-based)
    #[allow(dead_code)]
    pub start_line: usize,

    /// Line number where the end marker appears (1-based)
    #[allow(dead_code)]
    pub end_line: usize,

    /// Options parsed from the start marker
    pub options: TocOptions,

    /// The raw start marker text for debugging
    #[allow(dead_code)]
    pub raw_start_marker: String,
}

impl TocRegionSpec {
    pub fn new(
        start_span: TextSpan,
        end_span: TextSpan,
        start_line: usize,
        end_line: usize,
        raw_start_marker: String,
    ) -> Self {
        Self {
            start_span,
            end_span,
            start_line,
            end_line,
            options: TocOptions::default(),
            raw_start_marker,
        }
    }

    pub fn with_options(mut self, options: TocOptions) -> Self {
        self.options = options;
        self
    }

    /// Get the span of content between start and end markers
    pub fn content_span(&self) -> TextSpan {
        TextSpan::new(self.start_span.end, self.end_span.start)
    }
}

/// Represents a heading found in a Markdown file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    /// Heading level (1-6, corresponding to # through ######)
    pub level: u8,

    /// The text content of the heading
    pub text: String,

    /// Generated anchor/slug for linking
    pub anchor: String,

    /// Line number where the heading appears (1-based)
    pub line: usize,
}

impl Heading {
    pub fn new(level: u8, text: String, line: usize) -> Self {
        let anchor = generate_anchor(&text);
        Self {
            level,
            text,
            anchor,
            line,
        }
    }

    #[allow(dead_code)]
    pub fn with_anchor(mut self, anchor: String) -> Self {
        self.anchor = anchor;
        self
    }

    /// Get the markdown link for this heading
    pub fn to_link(&self) -> String {
        format!("[{}](#{})", self.text, self.anchor)
    }
}

/// Generate a GitHub-style anchor from heading text
pub fn generate_anchor(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                // Remove other characters
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Ensure unique anchors by appending numbers if necessary
pub fn make_anchors_unique(headings: &mut [Heading]) {
    use std::collections::HashMap;

    let mut anchor_counts: HashMap<String, usize> = HashMap::new();

    for heading in headings.iter_mut() {
        let base_anchor = heading.anchor.clone();
        let count = anchor_counts.entry(base_anchor.clone()).or_insert(0);

        if *count > 0 {
            heading.anchor = format!("{}-{}", base_anchor, count);
        }

        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toc_style_from_str() {
        assert_eq!(TocStyle::from_str("bullet"), Some(TocStyle::Bullet));
        assert_eq!(TocStyle::from_str("Bullet"), Some(TocStyle::Bullet));
        assert_eq!(TocStyle::from_str("numbered"), Some(TocStyle::Numbered));
        assert_eq!(TocStyle::from_str("NUMBERED"), Some(TocStyle::Numbered));
        assert_eq!(TocStyle::from_str("invalid"), None);
    }

    #[test]
    fn test_toc_options_includes_level() {
        let opts = TocOptions::default();
        assert!(!opts.includes_level(1));
        assert!(opts.includes_level(2));
        assert!(opts.includes_level(4));
        assert!(opts.includes_level(6));
        assert!(!opts.includes_level(7));
    }

    #[test]
    fn test_toc_options_builder() {
        let opts = TocOptions::new()
            .with_from_level(1)
            .with_to_level(3)
            .with_style(TocStyle::Numbered);

        assert_eq!(opts.from_level, 1);
        assert_eq!(opts.to_level, 3);
        assert_eq!(opts.style, TocStyle::Numbered);
    }

    #[test]
    fn test_heading_new() {
        let heading = Heading::new(2, "My Heading".to_string(), 10);
        assert_eq!(heading.level, 2);
        assert_eq!(heading.text, "My Heading");
        assert_eq!(heading.anchor, "my-heading");
        assert_eq!(heading.line, 10);
    }

    #[test]
    fn test_heading_to_link() {
        let heading = Heading::new(2, "Getting Started".to_string(), 5);
        assert_eq!(heading.to_link(), "[Getting Started](#getting-started)");
    }

    #[test]
    fn test_generate_anchor() {
        assert_eq!(generate_anchor("Simple"), "simple");
        assert_eq!(generate_anchor("With Spaces"), "with-spaces");
        assert_eq!(generate_anchor("With-Dashes"), "with-dashes");
        assert_eq!(generate_anchor("API v2.0"), "api-v20");
        assert_eq!(generate_anchor("Hello, World!"), "hello-world");
        assert_eq!(generate_anchor("Multiple   Spaces"), "multiple-spaces");
    }

    #[test]
    fn test_make_anchors_unique() {
        let mut headings = vec![
            Heading::new(2, "Introduction".to_string(), 1),
            Heading::new(2, "Introduction".to_string(), 10),
            Heading::new(2, "Usage".to_string(), 20),
            Heading::new(2, "Introduction".to_string(), 30),
        ];

        make_anchors_unique(&mut headings);

        assert_eq!(headings[0].anchor, "introduction");
        assert_eq!(headings[1].anchor, "introduction-1");
        assert_eq!(headings[2].anchor, "usage");
        assert_eq!(headings[3].anchor, "introduction-2");
    }

    #[test]
    fn test_toc_region_content_span() {
        let region = TocRegionSpec::new(
            TextSpan::new(0, 50),
            TextSpan::new(100, 120),
            1,
            5,
            "<!-- mk-toc:start -->".to_string(),
        );

        let content = region.content_span();
        assert_eq!(content.start, 50);
        assert_eq!(content.end, 100);
    }
}
