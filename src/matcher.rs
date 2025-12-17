use std::{fs::read_to_string, io::Error};

pub trait Matcher {
    fn find(&self, text: &str) -> Option<(usize, usize)>;
    
    fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }
}

pub struct LiteralMatcher<'l> {
    pub pattern: &'l str,
}

impl<'l> LiteralMatcher<'l> {
    pub fn new(pattern: &'l str) -> Self {
        Self { pattern: pattern }
    }    
}

impl<'l> Matcher for LiteralMatcher<'l> {    
    fn find(&self, text: &str) -> Option<(usize, usize)> {
        text.find(&self.pattern)
            .map(|start| (start +1, text.len()))
    }
}

pub fn read_lines(file: &str) -> Result<Vec<String>, Error> {
    let lines_read = read_to_string(file)?;
    
    Ok(
        lines_read
            .lines()
            .map(|x| String::from(x))
            .collect::<Vec<String>>()
    )
}
