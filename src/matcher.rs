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
