// src/lib.rs

//! # VelvetIO - Simple CLI Input for Rust
//!
//! Getting user input in Rust CLI apps is annoying. VelvetIO makes it simple.
//!
//! ```no_run
//! use velvetio::prelude::*;
//!
//! let name = ask!("What's your name?");
//! let age = ask!("How old are you?" => u32);
//!
//! if confirm!("Do you like Rust?") {
//!     println!("Great choice, {}!", name);
//! }
//! ```
//!
//! For complex input, use forms:
//! ```no_run
//! use velvetio::prelude::*;
//!
//! let user_data = form()
//!     .text("name", "Full name")
//!     .number("age", "Age")
//!     .choice("role", "Role", &["User", "Admin"])
//!     .collect();
//! ```

mod core;
mod error;
mod parser;
mod validators;

pub use core::{
    ask, ask_with_default, ask_with_validation, choose, confirm, form, multi_select, try_ask,
};
pub use error::{Result, VelvetIOError};
pub use parser::Parse;
pub use validators::{and, in_range, is_positive, max_length, min_length, not_empty, or};

/// Main macro for getting input
#[macro_export]
macro_rules! ask {
    ($prompt:expr) => {
        $crate::ask::<String>($prompt)
    };
    ($prompt:expr => $type:ty) => {
        $crate::ask::<$type>($prompt)
    };
    ($prompt:expr, validate: $validator:expr) => {
        $crate::ask_with_validation::<String, _>($prompt, $validator, None)
    };
    ($prompt:expr => $type:ty, validate: $validator:expr) => {
        $crate::ask_with_validation::<$type, _>($prompt, $validator, None)
    };
    ($prompt:expr, validate: $validator:expr, error: $error_msg:expr) => {
        $crate::ask_with_validation::<String, _>($prompt, $validator, Some($error_msg))
    };
    ($prompt:expr => $type:ty, validate: $validator:expr, error: $error_msg:expr) => {
        $crate::ask_with_validation::<$type, _>($prompt, $validator, Some($error_msg))
    };
    ($prompt:expr, default: $default:expr) => {
        $crate::ask_with_default($prompt, $default)
    };
    ($prompt:expr => $type:ty, default: $default:expr) => {
        $crate::ask_with_default($prompt, $default)
    };
    ($prompt:expr, or: $default:expr) => {
        $crate::try_ask::<String>($prompt).unwrap_or($default.into())
    };
    ($prompt:expr => $type:ty, or: $default:expr) => {
        $crate::try_ask::<$type>($prompt).unwrap_or($default)
    };
}

#[macro_export]
macro_rules! try_ask {
    ($prompt:expr) => {
        $crate::try_ask::<String>($prompt)
    };
    ($prompt:expr => $type:ty) => {
        $crate::try_ask::<$type>($prompt)
    };
}

#[macro_export]
macro_rules! confirm {
    ($prompt:expr) => {
        $crate::confirm($prompt)
    };
}

#[macro_export]
macro_rules! choose {
    ($prompt:expr, [$($choice:expr),+ $(,)?]) => {
        $crate::choose($prompt, &[$($choice),+])
    };

    ($prompt:expr, $choices:expr) => {
        $crate::choose($prompt, $choices.as_ref())
    };
}

#[macro_export]
macro_rules! multi_select {
    ($prompt:expr, [$($choice:expr),+ $(,)?]) => {
        $crate::multi_select($prompt, &[$($choice),+])
    };

    ($prompt:expr, $choices:expr) => {
        $crate::multi_select($prompt, $choices.as_ref())
    };

}

/// Quick form macro for simple cases
#[macro_export]
macro_rules! quick_form {
    {
        $($key:expr => $prompt:expr),+ $(,)?
    } => {{
        let mut form_data = std::collections::HashMap::new();
        $(
            let value = $crate::ask::<String>($prompt);
            form_data.insert($key.to_string(), value);
        )+
        form_data
    }};
}

/// Add Parse impl for types that have FromStr
#[macro_export]
macro_rules! quick_parse {
    ($type:ty) => {
        impl $crate::Parse for $type {
            fn parse(input: &str) -> $crate::Result<Self> {
                input
                    .trim()
                    .parse()
                    .map_err(|_| $crate::VelvetIOError::parse_error(input, stringify!($type)))
            }

            fn type_name() -> &'static str {
                stringify!($type)
            }
        }
    };
}

pub mod prelude {
    pub use crate::{
        Parse, Result, VelvetIOError, ask, choose, confirm, form, multi_select, quick_form,
        quick_parse, try_ask,
    };
    pub use crate::{and, in_range, is_positive, max_length, min_length, not_empty, or};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_trait_exports() {
        assert_eq!(<String as Parse>::type_name(), "text");
        assert_eq!(<u32 as Parse>::type_name(), "positive integer");
        assert_eq!(<bool as Parse>::type_name(), "boolean");
    }

    #[test]
    fn test_error_creation() {
        let error = VelvetIOError::new("test", "input", "expected");
        assert_eq!(error.message, "test");

        let result: Result<String> = Ok("success".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validators() {
        assert!(not_empty(&"hello".to_string()));
        assert!(!not_empty(&"".to_string()));
        assert!(min_length(3)(&"hello".to_string()));
        assert!(!min_length(10)(&"hello".to_string()));
        assert!(is_positive(&42));
        assert!(!is_positive(&0));
    }

    #[test]
    fn test_form_builder_creation() {
        let _form = form()
            .text("name", "Name")
            .number("age", "Age")
            .boolean("active", "Active?")
            .choice("role", "Role", &["User", "Admin"])
            .optional("bio", "Bio");
    }

    #[test]
    fn test_quick_parse_macro() {
        #[derive(Debug, PartialEq)]
        struct TestType(String);

        impl std::str::FromStr for TestType {
            type Err = ();
            fn from_str(s: &str) -> std::result::Result<Self, ()> {
                Ok(TestType(s.to_string()))
            }
        }

        quick_parse!(TestType);

        let result = TestType::parse("hello").unwrap();
        assert_eq!(result, TestType("hello".to_string()));
        assert_eq!(TestType::type_name(), "TestType");
    }

    #[test]
    fn test_prelude_imports() {
        use crate::prelude::*;

        let _error = VelvetIOError::new("test", "input", "expected");
        let _result: Result<String> = Ok("test".to_string());
        assert!(not_empty(&"hello".to_string()));

        let _form = form().text("test", "Test field");

        // Don't actually run this since it would require input
        // let form_data = quick_form! {
        //     "name" => "Name",
        //     "age" => "Age",
        // };
        // assert_eq!(form_data.len(), 2);
    }
}
