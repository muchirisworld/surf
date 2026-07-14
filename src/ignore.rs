use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct IgnoreSet {
    patterns: Vec<IgnorePattern>,
}

struct IgnorePattern(String);

impl IgnoreSet {
    pub fn empty() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn from_file(path: &Path) -> io::Result<Self> {
        let line = fs::read_to_string(path)?;
        Ok(Self::from_text(line.as_str()))
    }

    pub fn from_text(text: &str) -> Self {
        Self {
            patterns: text
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with('#'))
                .map(|line| IgnorePattern(line.to_string()))
                .collect(),
        }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        self.patterns.iter().any(|x| {
            let ptn_path = Path::new(&x.0);
            ptn_path == path || path.starts_with(ptn_path)
        })
    }
}

#[allow(unused)]
impl IgnorePattern {
    pub fn matches(&self, path: &Path) -> bool {
        let pattern = self.0.as_str();

        if let Some(dir) = pattern.strip_suffix('/') {
            return path.components().any(|x| x.as_os_str() == dir);
        }

        if let Some(suffix) = pattern.strip_prefix("*.") {
            return path
                .file_name()
                .and_then(|x| x.to_str())
                .is_some_and(|x| x.ends_with(suffix));
        }

        path.components().any(|part| part.as_os_str() == pattern)
            || path.file_name().and_then(|name| name.to_str()) == Some(pattern)
    }
}

pub fn default_ignore_path(root: &Path) -> PathBuf {
    root.join(".surfignore")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore_exact_and_prefix() {
        let text = "target\n*.log\n# this is a comment\n\n  Cargo.lock  \n";
        let ignore = IgnoreSet::from_text(text);

        assert_eq!(ignore.patterns.len(), 3);
        assert_eq!(ignore.patterns[0].0, "target");
        assert_eq!(ignore.patterns[1].0, "*.log");
        assert_eq!(ignore.patterns[2].0, "Cargo.lock");

        assert!(ignore.is_ignored(Path::new("target")));
        assert!(ignore.is_ignored(Path::new("target/debug/binary")));
        assert!(ignore.is_ignored(Path::new("Cargo.lock")));
        assert!(!ignore.is_ignored(Path::new("src/main.rs")));
    }

    #[test]
    fn test_ignore_pattern_matches() {
        // Test custom IgnorePattern::matches logic if needed, although is_ignored is the primary API.
        // We'll test directory suffix matching
        let pattern = IgnorePattern("target/".to_string());
        assert!(pattern.matches(Path::new("target")));
        assert!(pattern.matches(Path::new("project/target/bin")));

        let p_glob = IgnorePattern("*.rs".to_string());
        assert!(p_glob.matches(Path::new("src/main.rs")));
        assert!(!p_glob.matches(Path::new("src/main.txt")));
    }
}
