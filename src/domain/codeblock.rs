use std::path::PathBuf;

/// Represents a span of text in a file using byte offsets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextSpan {
    /// Starting byte offset (inclusive)
    pub start: usize,
    /// Ending byte offset (exclusive)
    pub end: usize,
}

impl TextSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

/// Represents a range of lines in a source file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineRange {
    /// 1-based inclusive start line
    pub start: usize,
    /// 1-based inclusive end line
    pub end: usize,
}

impl LineRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Check if a line number (1-based) is within this range
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }
}

/// Specification for a code block that needs to be synchronized
#[derive(Debug, Clone)]
pub struct CodeblockSpec {
    /// Byte offsets of the marker comment in the Markdown file
    pub marker_span: TextSpan,

    /// Line number where the marker comment appears (1-based)
    pub marker_line: usize,

    /// Path to the source file (resolved)
    pub source_path: PathBuf,

    /// Language identifier for the code fence (explicit or inferred)
    pub lang: Option<String>,

    /// Optional line range to extract from the source file
    pub line_range: Option<LineRange>,

    /// Number of leading spaces to remove from each line
    pub dedent: Option<usize>,

    /// Span of the existing fenced code block, if present
    pub existing_block_span: Option<TextSpan>,

    /// The raw marker text for debugging/error messages
    #[allow(dead_code)]
    pub raw_marker: String,
}

impl CodeblockSpec {
    pub fn new(
        marker_span: TextSpan,
        marker_line: usize,
        source_path: PathBuf,
        raw_marker: String,
    ) -> Self {
        Self {
            marker_span,
            marker_line,
            source_path,
            lang: None,
            line_range: None,
            dedent: None,
            existing_block_span: None,
            raw_marker,
        }
    }

    pub fn with_lang(mut self, lang: Option<String>) -> Self {
        self.lang = lang;
        self
    }

    pub fn with_line_range(mut self, line_range: Option<LineRange>) -> Self {
        self.line_range = line_range;
        self
    }

    pub fn with_dedent(mut self, dedent: Option<usize>) -> Self {
        self.dedent = dedent;
        self
    }

    pub fn with_existing_block(mut self, existing_block_span: Option<TextSpan>) -> Self {
        self.existing_block_span = existing_block_span;
        self
    }
}

/// Options parsed from a mk-code marker comment
#[derive(Debug, Clone, Default)]
pub struct CodeblockOptions {
    pub lang: Option<String>,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub dedent: Option<usize>,
    pub region: Option<String>,
}

impl CodeblockOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn line_range(&self) -> Option<LineRange> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Some(LineRange::new(start, end)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_span() {
        let span = TextSpan::new(10, 20);
        assert_eq!(span.len(), 10);
        assert!(!span.is_empty());

        let empty_span = TextSpan::new(10, 10);
        assert!(empty_span.is_empty());
    }

    #[test]
    fn test_line_range() {
        let range = LineRange::new(5, 10);
        assert!(range.contains(5));
        assert!(range.contains(7));
        assert!(range.contains(10));
        assert!(!range.contains(4));
        assert!(!range.contains(11));
    }

    #[test]
    fn test_codeblock_spec_builder() {
        let spec = CodeblockSpec::new(
            TextSpan::new(0, 10),
            1,
            PathBuf::from("test.rs"),
            "<!-- mk-code: test.rs -->".to_string(),
        )
        .with_lang(Some("rust".to_string()))
        .with_line_range(Some(LineRange::new(1, 10)))
        .with_dedent(Some(4));

        assert_eq!(spec.lang, Some("rust".to_string()));
        assert_eq!(spec.line_range, Some(LineRange::new(1, 10)));
        assert_eq!(spec.dedent, Some(4));
    }

    #[test]
    fn test_codeblock_options_line_range() {
        let mut opts = CodeblockOptions::new();
        assert!(opts.line_range().is_none());

        opts.start = Some(5);
        opts.end = Some(10);
        let range = opts.line_range().unwrap();
        assert_eq!(range.start, 5);
        assert_eq!(range.end, 10);
    }
}
