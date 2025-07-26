// src/error.rs

use std::fmt;

pub type Result<T> = std::result::Result<T, VelvetIOError>;

/// Error type for VelvetIO operations
#[derive(Debug, Clone)]
pub struct VelvetIOError {
    pub message: String,
    pub input: String,
    pub expected: String,
}

impl VelvetIOError {
    pub fn new(
        message: impl Into<String>,
        input: impl Into<String>,
        expected: impl Into<String>,
    ) -> Self {
        Self {
            message: message.into(),
            input: input.into(),
            expected: expected.into(),
        }
    }

    /// Create parse error - "Cannot parse 'abc' as number"
    pub fn parse_error(input: impl Into<String>, expected_type: impl Into<String>) -> Self {
        let input = input.into();
        let expected_type = expected_type.into();

        Self {
            message: format!("Cannot parse '{}' as {}", input, expected_type),
            input,
            expected: expected_type,
        }
    }

    /// Create validation error with custom message
    pub fn validation_error(input: impl Into<String>, custom_message: impl Into<String>) -> Self {
        Self {
            message: custom_message.into(),
            input: input.into(),
            expected: "valid input".to_string(),
        }
    }
}

impl fmt::Display for VelvetIOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for VelvetIOError {}

impl From<std::io::Error> for VelvetIOError {
    fn from(error: std::io::Error) -> Self {
        Self {
            message: format!("Input error: {}", error),
            input: String::new(),
            expected: "valid input".to_string(),
        }
    }
}
