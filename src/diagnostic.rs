use std::fmt;

pub enum ExitCode {
    Success = 0,
    Failure = 1,
    Usage = 2,
}

pub struct Diagnostic {
    pub code: ExitCode,
    pub message: String,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn usage(message: impl Into<String>) -> Self {
        Self {
            code: ExitCode::Usage,
            message: message.into(),
            help: Some("usage: surf [OPTIONS] <pattern> <path>...".to_string()),
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            code: ExitCode::Failure,
            message: message.into(),
            help: None,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "error: {}", self.message)?;
        if let Some(help) = &self.help {
            writeln!(f, "help: {help}")?;
        }
        Ok(())
    }
}