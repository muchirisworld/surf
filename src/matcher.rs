#[derive(Debug, Clone, Copy)]
pub enum MatchMode {
    WholeLine,
    Contains,
}

pub struct Matcher {
    pattern: String,
    folded_pattern: Option<String>,
    ignore_case: bool,
    mode: MatchMode,
    invert: bool,
}

impl Matcher {
    pub fn new(pattern: String, ignore_case: bool, mode: MatchMode, invert: bool) -> Self {
        let folded_pattern = ignore_case.then(|| pattern.to_lowercase());
        Self {
            pattern,
            folded_pattern,
            ignore_case,
            mode,
            invert,
        }
    }

    fn match_against(&self, line: &str, pattern: &str) -> bool {
        match self.mode {
            MatchMode::Contains => line.contains(pattern),
            MatchMode::WholeLine => line == pattern,
        }
    }

    pub fn is_match(&self, line: &str) -> bool {
        let matched = if self.ignore_case {
            self.match_against(&line.to_lowercase(), self.folded_pattern.as_ref().unwrap())
        } else {
            self.match_against(line, &self.pattern)
        };

        if self.invert { !matched } else { matched }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_case_sensitive() {
        let matcher = Matcher::new("rust".to_string(), false, MatchMode::Contains, false);
        assert!(matcher.is_match("rust programming"));
        assert!(matcher.is_match("rust"));
        assert!(!matcher.is_match("Rust"));
        assert!(!matcher.is_match("cargo"));
    }

    #[test]
    fn test_contains_case_insensitive() {
        let matcher = Matcher::new("rUsT".to_string(), true, MatchMode::Contains, false);
        assert!(matcher.is_match("rust programming"));
        assert!(matcher.is_match("RUST"));
        assert!(matcher.is_match("Rust"));
        assert!(!matcher.is_match("cargo"));
    }

    #[test]
    fn test_whole_line() {
        let matcher = Matcher::new("rust".to_string(), false, MatchMode::WholeLine, false);
        assert!(matcher.is_match("rust"));
        assert!(!matcher.is_match("rust programming"));
    }

    #[test]
    fn test_invert_match() {
        let matcher = Matcher::new("rust".to_string(), false, MatchMode::Contains, true);
        assert!(!matcher.is_match("rust programming"));
        assert!(matcher.is_match("cargo"));
    }
}
