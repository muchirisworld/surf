use crate::matcher::Matcher;
use std::collections::VecDeque;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    pub before: usize,
    pub after: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchEvent {
    Match { line_number: usize, line: String },
    Context { line_number: usize, line: String },
    Separator,
}

pub fn search_reader<R>(
    reader: R,
    matcher: &Matcher,
    context: Context,
) -> io::Result<Vec<SearchEvent>>
where
    R: BufRead,
{
    let mut events = Vec::new();
    let mut before = VecDeque::new();
    let mut remaining_after = 0usize;
    let mut emitted_any = false;

    for (index, line) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line = line?;

        if matcher.is_match(&line) {
            if emitted_any && remaining_after == 0 && context.before > 0 {
                events.push(SearchEvent::Separator);
            }

            while let Some((line_number, line)) = before.pop_front() {
                events.push(SearchEvent::Context { line_number, line });
            }

            events.push(SearchEvent::Match { line_number, line });
            emitted_any = true;
            remaining_after = context.after;
        } else if remaining_after > 0 {
            events.push(SearchEvent::Context { line_number, line });
            remaining_after -= 1;
        } else {
            before.push_back((line_number, line));
            while before.len() > context.before {
                before.pop_front();
            }
        }
    }

    Ok(events)
}
