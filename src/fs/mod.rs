use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub mod path_utils;

/// Discover Markdown files from the given paths
pub fn discover_markdown_files(paths: &[PathBuf], glob_pattern: &str) -> Result<Vec<PathBuf>> {
    let mut markdown_files = Vec::new();

    for path in paths {
        if path.is_file() {
            // If it's a file, add it directly
            markdown_files.push(path.clone());
        } else if path.is_dir() {
            // If it's a directory, walk it and find matching files
            let files = find_files_in_dir(path, glob_pattern)?;
            markdown_files.extend(files);
        } else {
            anyhow::bail!("Path does not exist: {}", path.display());
        }
    }

    Ok(markdown_files)
}

/// Find files in a directory matching a glob pattern
fn find_files_in_dir(dir: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    // For simple patterns like "**/*.md", we can use walkdir directly
    if pattern == "**/*.md" || pattern == "*.md" {
        for entry in WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                files.push(path.to_path_buf());
            }
        }
    } else {
        // For more complex patterns, use the glob crate
        let pattern_str = dir.join(pattern).to_string_lossy().to_string();
        for entry in glob::glob(&pattern_str)
            .with_context(|| format!("Invalid glob pattern: {}", pattern))?
        {
            let path = entry.with_context(|| "Failed to read glob entry")?;
            if path.is_file() {
                files.push(path);
            }
        }
    }

    files.sort();
    Ok(files)
}

/// Read a file's contents as UTF-8
pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path.display()))
}

/// Write content to a file
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("Failed to write file: {}", path.display()))
}

/// Create a backup of a file with a .bak extension
pub fn create_backup(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let backup_path = path.with_extension(format!(
        "{}.bak",
        path.extension().and_then(|s| s.to_str()).unwrap_or("")
    ));

    fs::copy(path, &backup_path)
        .with_context(|| format!("Failed to create backup at: {}", backup_path.display()))?;

    log::debug!("Created backup: {}", backup_path.display());
    Ok(())
}

/// Write a file with optional backup
pub fn write_file_with_backup(path: &Path, content: &str, create_backup: bool) -> Result<()> {
    if create_backup && path.exists() {
        self::create_backup(path)?;
    }
    write_file(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_read_write_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.md");

        let content = "# Hello World\n\nThis is a test.";
        write_file(&file_path, content).unwrap();

        let read_content = read_file(&file_path).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_create_backup() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.md");
        let backup_path = dir.path().join("test.md.bak");

        write_file(&file_path, "original content").unwrap();
        create_backup(&file_path).unwrap();

        assert!(backup_path.exists());
        let backup_content = read_file(&backup_path).unwrap();
        assert_eq!(backup_content, "original content");
    }

    #[test]
    fn test_write_file_with_backup() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.md");

        write_file(&file_path, "original").unwrap();
        write_file_with_backup(&file_path, "updated", true).unwrap();

        let content = read_file(&file_path).unwrap();
        assert_eq!(content, "updated");

        let backup_path = dir.path().join("test.md.bak");
        assert!(backup_path.exists());
        let backup_content = read_file(&backup_path).unwrap();
        assert_eq!(backup_content, "original");
    }

    #[test]
    fn test_discover_markdown_files() {
        let dir = tempdir().unwrap();

        // Create some test files
        fs::create_dir_all(dir.path().join("subdir")).unwrap();
        write_file(&dir.path().join("test1.md"), "content1").unwrap();
        write_file(&dir.path().join("subdir/test2.md"), "content2").unwrap();
        write_file(&dir.path().join("test.txt"), "not markdown").unwrap();

        let files = discover_markdown_files(&[dir.path().to_path_buf()], "**/*.md").unwrap();

        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("test1.md")));
        assert!(files.iter().any(|p| p.ends_with("test2.md")));
    }

    #[test]
    fn test_discover_single_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("single.md");
        write_file(&file_path, "content").unwrap();

        let files = discover_markdown_files(&[file_path.clone()], "**/*.md").unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);
    }
}
