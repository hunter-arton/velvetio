// src/parser.rs

use crate::error::{Result, VelvetIOError};

/// Parse strings into Rust types
pub trait Parse: Sized {
    fn parse(input: &str) -> Result<Self>;
    fn type_name() -> &'static str;
}

impl Parse for String {
    fn parse(input: &str) -> Result<Self> {
        Ok(input.to_string())
    }

    fn type_name() -> &'static str {
        "text"
    }
}

// Accept many ways to say yes/no
impl Parse for bool {
    fn parse(input: &str) -> Result<Self> {
        match input.trim().to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "1" | "on" => Ok(true),
            "false" | "f" | "no" | "n" | "0" | "off" => Ok(false),
            _ => Err(VelvetIOError::parse_error(
                input,
                "boolean (yes/no, true/false, y/n, 1/0)",
            )),
        }
    }

    fn type_name() -> &'static str {
        "boolean"
    }
}

impl Parse for char {
    fn parse(input: &str) -> Result<Self> {
        let trimmed = input.trim();
        let mut chars = trimmed.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(VelvetIOError::parse_error(input, "single character")),
        }
    }

    fn type_name() -> &'static str {
        "character"
    }
}

// Empty or "none" becomes None, otherwise parse as T
impl<T: Parse> Parse for Option<T> {
    fn parse(input: &str) -> Result<Self> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Ok(None);
        }

        match trimmed.to_lowercase().as_str() {
            "none" | "null" | "nil" | "-" | "skip" => Ok(None),
            _ => match T::parse(trimmed) {
                Ok(value) => Ok(Some(value)),
                Err(e) => Err(e),
            },
        }
    }

    fn type_name() -> &'static str {
        "optional value (or empty/none for no value)"
    }
}

// Generate Parse impls for all numeric types
macro_rules! impl_numeric {
    ($type:ty, $name:expr) => {
        impl Parse for $type {
            fn parse(input: &str) -> Result<Self> {
                input
                    .trim()
                    .parse::<$type>()
                    .map_err(|_| VelvetIOError::parse_error(input, $name))
            }

            fn type_name() -> &'static str {
                $name
            }
        }
    };
}

impl_numeric!(i8, "integer (-128 to 127)");
impl_numeric!(i16, "integer (-32,768 to 32,767)");
impl_numeric!(i32, "integer");
impl_numeric!(i64, "integer");
impl_numeric!(i128, "integer");
impl_numeric!(isize, "integer");

impl_numeric!(u8, "positive integer (0 to 255)");
impl_numeric!(u16, "positive integer (0 to 65,535)");
impl_numeric!(u32, "positive integer");
impl_numeric!(u64, "positive integer");
impl_numeric!(u128, "positive integer");
impl_numeric!(usize, "positive integer");

impl_numeric!(f32, "decimal number");
impl_numeric!(f64, "decimal number");

// Smart separator detection: comma, semicolon, pipe, or space
impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &str) -> Result<Self> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        let separator = if trimmed.contains(',') {
            ','
        } else if trimmed.contains(';') {
            ';'
        } else if trimmed.contains('|') {
            '|'
        } else {
            ' '
        };

        let parts: Vec<&str> = if separator == ' ' {
            trimmed.split_whitespace().collect()
        } else {
            trimmed
                .split(separator)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect()
        };

        let mut results = Vec::with_capacity(parts.len());
        for part in parts {
            match T::parse(part) {
                Ok(value) => results.push(value),
                Err(_) => {
                    return Err(VelvetIOError::parse_error(
                        input,
                        &format!("list of {}", T::type_name()),
                    ));
                }
            }
        }

        Ok(results)
    }

    fn type_name() -> &'static str {
        "list of values"
    }
}

impl<T1: Parse, T2: Parse> Parse for (T1, T2) {
    fn parse(input: &str) -> Result<Self> {
        let parts: Vec<&str> = if input.contains(',') {
            input.split(',').map(|s| s.trim()).collect()
        } else {
            input.split_whitespace().collect()
        };

        if parts.len() != 2 {
            return Err(VelvetIOError::parse_error(
                input,
                "pair of values (separate with comma or space)",
            ));
        }

        let first = T1::parse(parts[0]).map_err(|_| {
            VelvetIOError::parse_error(
                input,
                &format!("pair: {} and {}", T1::type_name(), T2::type_name()),
            )
        })?;

        let second = T2::parse(parts[1]).map_err(|_| {
            VelvetIOError::parse_error(
                input,
                &format!("pair: {} and {}", T1::type_name(), T2::type_name()),
            )
        })?;

        Ok((first, second))
    }

    fn type_name() -> &'static str {
        "pair of values"
    }
}

impl<T1: Parse, T2: Parse, T3: Parse> Parse for (T1, T2, T3) {
    fn parse(input: &str) -> Result<Self> {
        let parts: Vec<&str> = if input.contains(',') {
            input.split(',').map(|s| s.trim()).collect()
        } else {
            input.split_whitespace().collect()
        };

        if parts.len() != 3 {
            return Err(VelvetIOError::parse_error(
                input,
                "triple of values (separate with comma or space)",
            ));
        }

        let first = T1::parse(parts[0])
            .map_err(|_| VelvetIOError::parse_error(input, "triple of values"))?;

        let second = T2::parse(parts[1])
            .map_err(|_| VelvetIOError::parse_error(input, "triple of values"))?;

        let third = T3::parse(parts[2])
            .map_err(|_| VelvetIOError::parse_error(input, "triple of values"))?;

        Ok((first, second, third))
    }

    fn type_name() -> &'static str {
        "triple of values"
    }
}
