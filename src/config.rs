use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{matcher, render};

pub struct Settings {
    pub recursive: bool,
    pub line_numbers: bool,
    pub ignore_case: bool,
    pub invert_match: bool,
    pub mode: matcher::MatchMode,
    pub before_context: usize,
    pub after_context: usize,
    pub color: render::Color,
    pub ignore_file: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileConfig {
    pub recursive: Option<bool>,
    pub line_numbers: Option<bool>,
    pub ignore_case: Option<bool>,
    pub invert_match: Option<bool>,
    pub whole_line: Option<bool>,
    pub before_context: Option<usize>,
    pub after_context: Option<usize>,
    pub color: Option<String>,
    pub ignore_file: Option<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            recursive: false,
            line_numbers: false,
            ignore_case: false,
            invert_match: false,
            mode: matcher::MatchMode::Contains,
            before_context: 0,
            after_context: 0,
            color: render::Color::Auto,
            ignore_file: None,
        }
    }
}

pub fn load_file(path: &Path) -> Result<FileConfig, String> {
    let text = fs::read_to_string(path)
        .map_err(|err| format!("failed to read config {}: {err}", path.display()))?;
    toml::from_str(&text).map_err(|err| format!("failed to parse config {}: {err}", path.display()))
}

#[allow(unused)]
fn apply_file(settings: &mut Settings, file: &FileConfig) -> Result<(), String> {
    settings.recursive = file.recursive.unwrap_or(settings.recursive);
    settings.line_numbers = file.line_numbers.unwrap_or(settings.line_numbers);
    settings.ignore_case = file.ignore_case.unwrap_or(settings.ignore_case);
    settings.invert_match = file.invert_match.unwrap_or(settings.invert_match);
    settings.before_context = file.before_context.unwrap_or(settings.before_context);
    settings.after_context = file.after_context.unwrap_or(settings.after_context);
    settings.ignore_file = file
        .ignore_file
        .clone()
        .or_else(|| settings.ignore_file.clone());

    if file.whole_line.unwrap_or(false) {
        settings.mode = matcher::MatchMode::WholeLine;
    }

    if let Some(color) = &file.color {
        settings.color = parse_color(color)?;
    }

    Ok(())
}

fn parse_color(color: &str) -> Result<render::Color, String> {
    match color {
        "auto" => Ok(render::Color::Auto),
        "always" => Ok(render::Color::Always),
        "never" => Ok(render::Color::Never),
        _ => Err(format!("invalid color value `{color}`")),
    }
}
