use crate::search::SearchEvent;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderOptions {
    pub line_numbers: bool,
    pub color: Color,
}

pub fn render_events<W>(
    mut writer: W,
    path: &Path,
    events: &[SearchEvent],
    options: RenderOptions,
) -> io::Result<()>
where
    W: Write,
{
    for event in events {
        match event {
            SearchEvent::Match { line_number, line } => {
                write_line(&mut writer, path, *line_number, line, true, options)?;
            }
            SearchEvent::Context { line_number, line } => {
                write_line(&mut writer, path, *line_number, line, false, options)?;
            }
            SearchEvent::Separator => {
                writeln!(writer, "--")?;
            }
        }
    }

    Ok(())
}

fn write_line<W>(
    writer: &mut W,
    path: &Path,
    line_number: usize,
    line: &str,
    is_match: bool,
    options: RenderOptions,
) -> io::Result<()>
where
    W: Write,
{
    let use_color = matches!(options.color, Color::Always);
    let marker = if is_match { ":" } else { "-" };

    write!(writer, "{}{}", path.display(), marker)?;
    if options.line_numbers {
        write!(writer, "{line_number}{marker}")?;
    }

    if is_match && use_color {
        writeln!(writer, "\x1b[31m{line}\x1b[0m")
    } else {
        writeln!(writer, "{line}")
    }
}
