use std::{fs::read_to_string, io::Error};

pub fn read_lines(file: &str) -> Result<Vec<String>, Error> {
    let lines_read = read_to_string(file)?;
    
    Ok(
        lines_read
            .lines()
            .map(|x| String::from(x))
            .collect::<Vec<String>>()
    )
}

pub fn find_matches<'a>(pattern: &str, content: &'a[String]) -> Vec<(usize, &'a str)> {
    content
        .iter()
        .enumerate()
        .filter(|(_, x)| x.contains(pattern))
        .map(|(idx, line)| (idx+1, line.as_str()))
        .collect()
}