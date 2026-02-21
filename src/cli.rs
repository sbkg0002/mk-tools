use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mk-tools",
    version,
    about = "A CLI tool for managing Markdown files"
)]
#[command(author, long_about = None)]
pub struct Cli {
    /// Reduce output verbosity
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Increase output verbosity (can be repeated: -v, -vv, -vvv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Show what would be done without writing files
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Control colored output
    #[arg(long, value_enum, default_value = "auto", global = true)]
    pub color: ColorChoice,

    /// Change working directory before running
    #[arg(short = 'C', long, global = true)]
    pub chdir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Sync code blocks in Markdown from source files
    Codeblocks(CodeblocksArgs),

    /// Generate/update table of contents in Markdown files
    Toc(TocArgs),

    /// Run validations (codeblocks + toc) in check mode
    Check(CheckArgs),

    /// Print version information
    Version,
}

#[derive(Parser, Debug)]
pub struct CodeblocksArgs {
    /// Files or directories to process
    #[arg(value_name = "PATHS")]
    pub paths: Vec<PathBuf>,

    /// Base directory for resolving source file references
    #[arg(long)]
    pub root: Option<PathBuf>,

    /// Glob pattern for Markdown files in directories
    #[arg(long, default_value = "**/*.md")]
    pub glob: String,

    /// Language overrides (e.g., "py=python,rs=rust")
    #[arg(long, value_parser = parse_language_overrides)]
    pub language_overrides: Option<Vec<(String, String)>>,

    /// Do not modify files; exit non-zero if changes would be made
    #[arg(long)]
    pub check: bool,

    /// Do not create backup files
    #[arg(long)]
    pub no_backup: bool,

    /// File encoding (only UTF-8 is supported currently)
    #[arg(long, default_value = "utf-8")]
    pub encoding: String,
}

#[derive(Parser, Debug)]
pub struct TocArgs {
    /// Files or directories to process
    #[arg(value_name = "PATHS")]
    pub paths: Vec<PathBuf>,

    /// Base directory for resolving paths
    #[arg(long)]
    pub root: Option<PathBuf>,

    /// Glob pattern for Markdown files in directories
    #[arg(long, default_value = "**/*.md")]
    pub glob: String,

    /// Do not modify files; exit non-zero if changes would be made
    #[arg(long)]
    pub check: bool,

    /// Do not create backup files
    #[arg(long)]
    pub no_backup: bool,

    /// File encoding (only UTF-8 is supported currently)
    #[arg(long, default_value = "utf-8")]
    pub encoding: String,
}

#[derive(Parser, Debug)]
pub struct CheckArgs {
    /// Files or directories to process
    #[arg(value_name = "PATHS")]
    pub paths: Vec<PathBuf>,

    /// Glob pattern for Markdown files in directories
    #[arg(long, default_value = "**/*.md")]
    pub glob: String,

    /// Base directory for resolving paths
    #[arg(long)]
    pub root: Option<PathBuf>,
}

/// Parse language overrides in the format "ext=lang,ext2=lang2"
fn parse_language_overrides(s: &str) -> Result<Vec<(String, String)>, String> {
    s.split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() != 2 {
                Err(format!("Invalid language override format: '{}'", pair))
            } else {
                Ok((parts[0].to_string(), parts[1].to_string()))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_language_overrides() {
        let result = parse_language_overrides("py=python,rs=rust").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("py".to_string(), "python".to_string()));
        assert_eq!(result[1], ("rs".to_string(), "rust".to_string()));
    }

    #[test]
    fn test_parse_language_overrides_invalid() {
        let result = parse_language_overrides("invalid");
        assert!(result.is_err());
    }
}
