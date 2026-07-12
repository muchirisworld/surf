use std::{fs, io, path::{Path, PathBuf}};


pub struct IgnoreSet {
    patterns: Vec<IgnorePattern>
}

struct IgnorePattern(String);

impl IgnoreSet {
    pub fn empty() -> Self {
        Self { patterns: Vec::new() }
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
                .collect()
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
                .is_some_and(|x| x.ends_with(suffix))
        }

        path.components().any(|part| part.as_os_str() == pattern)
            || path.file_name().and_then(|name| name.to_str()) == Some(pattern)
    }
}

pub fn default_ignore_path(root: &Path) -> PathBuf {
    root.join(".surfignore")
}
