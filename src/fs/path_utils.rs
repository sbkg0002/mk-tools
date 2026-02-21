use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resolve a path relative to a base directory
pub fn resolve_path(path: &Path, base: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

/// Get the parent directory of a file, or use current directory if none
#[allow(dead_code)]
pub fn get_base_dir(file_path: &Path) -> PathBuf {
    file_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Normalize a path by canonicalizing it if possible
#[allow(dead_code)]
pub fn normalize_path(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("Failed to normalize path: {}", path.display()))
}

/// Get the file extension as a string
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

/// Map file extension to language identifier for code fences
pub fn extension_to_language(ext: &str, overrides: &HashMap<String, String>) -> Option<String> {
    // Check user-provided overrides first
    if let Some(lang) = overrides.get(ext) {
        return Some(lang.clone());
    }

    // Default mappings
    let default_map: HashMap<&str, &str> = [
        ("rs", "rust"),
        ("py", "python"),
        ("js", "javascript"),
        ("ts", "typescript"),
        ("jsx", "jsx"),
        ("tsx", "tsx"),
        ("sh", "bash"),
        ("bash", "bash"),
        ("zsh", "zsh"),
        ("fish", "fish"),
        ("c", "c"),
        ("cpp", "cpp"),
        ("cc", "cpp"),
        ("cxx", "cpp"),
        ("h", "c"),
        ("hpp", "cpp"),
        ("go", "go"),
        ("java", "java"),
        ("kt", "kotlin"),
        ("swift", "swift"),
        ("rb", "ruby"),
        ("php", "php"),
        ("cs", "csharp"),
        ("fs", "fsharp"),
        ("scala", "scala"),
        ("clj", "clojure"),
        ("erl", "erlang"),
        ("ex", "elixir"),
        ("exs", "elixir"),
        ("hs", "haskell"),
        ("ml", "ocaml"),
        ("lua", "lua"),
        ("r", "r"),
        ("R", "r"),
        ("jl", "julia"),
        ("nim", "nim"),
        ("zig", "zig"),
        ("v", "v"),
        ("dart", "dart"),
        ("pl", "perl"),
        ("pm", "perl"),
        ("sql", "sql"),
        ("md", "markdown"),
        ("json", "json"),
        ("yaml", "yaml"),
        ("yml", "yaml"),
        ("toml", "toml"),
        ("xml", "xml"),
        ("html", "html"),
        ("htm", "html"),
        ("css", "css"),
        ("scss", "scss"),
        ("sass", "sass"),
        ("less", "less"),
        ("vim", "vim"),
        ("dockerfile", "dockerfile"),
        ("makefile", "makefile"),
        ("mk", "makefile"),
        ("cmake", "cmake"),
        ("proto", "protobuf"),
        ("graphql", "graphql"),
        ("gql", "graphql"),
        ("tf", "terraform"),
        ("hcl", "hcl"),
    ]
    .iter()
    .copied()
    .collect();

    default_map.get(ext).map(|s| s.to_string())
}

/// Build a HashMap from language override pairs
pub fn build_language_overrides(pairs: &[(String, String)]) -> HashMap<String, String> {
    pairs.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_path_relative() {
        let base = PathBuf::from("/home/user/project");
        let rel = Path::new("src/main.rs");
        let resolved = resolve_path(rel, &base);
        assert_eq!(resolved, PathBuf::from("/home/user/project/src/main.rs"));
    }

    #[test]
    fn test_resolve_path_absolute() {
        let base = PathBuf::from("/home/user/project");
        let abs = Path::new("/etc/config.txt");
        let resolved = resolve_path(abs, &base);
        assert_eq!(resolved, PathBuf::from("/etc/config.txt"));
    }

    #[test]
    fn test_get_base_dir() {
        let file = Path::new("/home/user/project/docs/README.md");
        let base = get_base_dir(file);
        assert_eq!(base, PathBuf::from("/home/user/project/docs"));
    }

    #[test]
    fn test_get_base_dir_no_parent() {
        let file = Path::new("README.md");
        let base = get_base_dir(file);
        assert_eq!(base, PathBuf::from("."));
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(get_extension(Path::new("test.rs")), Some("rs".to_string()));
        assert_eq!(
            get_extension(Path::new("test.tar.gz")),
            Some("gz".to_string())
        );
        assert_eq!(get_extension(Path::new("no_extension")), None);
    }

    #[test]
    fn test_extension_to_language_defaults() {
        let overrides = HashMap::new();

        assert_eq!(
            extension_to_language("rs", &overrides),
            Some("rust".to_string())
        );
        assert_eq!(
            extension_to_language("py", &overrides),
            Some("python".to_string())
        );
        assert_eq!(
            extension_to_language("js", &overrides),
            Some("javascript".to_string())
        );
        assert_eq!(extension_to_language("unknown", &overrides), None);
    }

    #[test]
    fn test_extension_to_language_overrides() {
        let mut overrides = HashMap::new();
        overrides.insert("py".to_string(), "python3".to_string());
        overrides.insert("custom".to_string(), "mylang".to_string());

        assert_eq!(
            extension_to_language("py", &overrides),
            Some("python3".to_string())
        );
        assert_eq!(
            extension_to_language("custom", &overrides),
            Some("mylang".to_string())
        );
        // Should still fall back to defaults for non-overridden extensions
        assert_eq!(
            extension_to_language("rs", &overrides),
            Some("rust".to_string())
        );
    }

    #[test]
    fn test_build_language_overrides() {
        let pairs = vec![
            ("py".to_string(), "python".to_string()),
            ("rs".to_string(), "rust".to_string()),
        ];
        let map = build_language_overrides(&pairs);

        assert_eq!(map.get("py"), Some(&"python".to_string()));
        assert_eq!(map.get("rs"), Some(&"rust".to_string()));
        assert_eq!(map.len(), 2);
    }
}
