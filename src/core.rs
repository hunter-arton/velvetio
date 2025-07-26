// src/core.rs

use crate::{Parse, Result};
use std::collections::HashMap;
use std::io::{self, Write};

/// Keep asking until we get valid input
pub fn ask<T: Parse>(prompt: &str) -> T {
    loop {
        print!("{}: ", prompt);
        let _ = io::stdout().flush();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match T::parse(input.trim()) {
                Ok(value) => return value,
                Err(e) => eprintln!("❌ {}", e),
            },
            Err(e) => eprintln!("❌ Input error: {}", e),
        }
    }
}

/// Try once, return Result instead of retrying
pub fn try_ask<T: Parse>(prompt: &str) -> Result<T> {
    print!("{}: ", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    T::parse(input.trim())
}

/// Ask with validation function
pub fn ask_with_validation<T: Parse, F>(
    prompt: &str,
    validator: F,
    error_message: Option<&str>,
) -> T
where
    F: Fn(&T) -> bool,
{
    let default_error = "Invalid input, please try again";
    let error_msg = error_message.unwrap_or(default_error);

    loop {
        print!("{}: ", prompt);
        let _ = io::stdout().flush();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match T::parse(input.trim()) {
                Ok(value) => {
                    if validator(&value) {
                        return value;
                    } else {
                        eprintln!("❌ {}", error_msg);
                    }
                }
                Err(e) => eprintln!("❌ {}", e),
            },
            Err(e) => eprintln!("❌ Input error: {}", e),
        }
    }
}

/// Ask with default - hit enter to use default
pub fn ask_with_default<T: Parse + std::fmt::Display + Clone>(prompt: &str, default: T) -> T {
    print!("{} [{}]: ", prompt, default);
    let _ = io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                default
            } else {
                T::parse(trimmed).unwrap_or(default)
            }
        }
        Err(_) => default,
    }
}

/// Yes/no question
pub fn confirm(prompt: &str) -> bool {
    ask::<bool>(&format!("{} (y/n)", prompt))
}

/// Pick one option from a list
pub fn choose<T>(prompt: &str, choices: &[T]) -> T
where
    T: std::fmt::Display + Clone,
{
    if choices.is_empty() {
        panic!("Cannot choose from empty list");
    }

    loop {
        println!("{}:", prompt);
        for (i, choice) in choices.iter().enumerate() {
            println!("  {}. {}", i + 1, choice);
        }

        match try_ask::<usize>(&format!("Choose (1-{})", choices.len())) {
            Ok(index) if index >= 1 && index <= choices.len() => {
                return choices[index - 1].clone();
            }
            Ok(_) => eprintln!("❌ Please choose between 1 and {}", choices.len()),
            Err(e) => eprintln!("❌ {}", e),
        }
    }
}

/// Pick multiple options from a list
pub fn multi_select<T>(prompt: &str, choices: &[T]) -> Vec<T>
where
    T: std::fmt::Display + Clone,
{
    if choices.is_empty() {
        return Vec::new();
    }

    loop {
        println!("{}:", prompt);
        for (i, choice) in choices.iter().enumerate() {
            println!("  {}. {}", i + 1, choice);
        }
        println!("Enter numbers separated by commas (e.g., 1,3,5) or 'all' or 'none':");

        let input = ask::<String>("Selection");
        let input = input.trim().to_lowercase();

        if input == "none" || input.is_empty() {
            return Vec::new();
        }

        if input == "all" {
            return choices.to_vec();
        }

        let parts: Vec<&str> = input.split(',').map(|s| s.trim()).collect();
        let mut selected = Vec::new();
        let mut valid = true;

        for part in parts {
            match part.parse::<usize>() {
                Ok(num) if num >= 1 && num <= choices.len() => {
                    selected.push(choices[num - 1].clone());
                }
                Ok(num) => {
                    eprintln!("❌ {} is not a valid option (1-{})", num, choices.len());
                    valid = false;
                    break;
                }
                Err(_) => {
                    eprintln!("❌ Please enter numbers separated by commas");
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            return selected;
        }
    }
}

/// Form builder for collecting multiple inputs
pub struct Form {
    fields: Vec<FormField>,
}

struct FormField {
    key: String,
    prompt: String,
    field_type: FieldType,
}

enum FieldType {
    Text,
    Number,
    Boolean,
    Choice(Vec<String>),
    MultiChoice(Vec<String>),
    Optional,
    ValidatedText {
        validator: Box<dyn Fn(&str) -> bool>,
        error_msg: String,
    },
}

impl Form {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn text(mut self, key: &str, prompt: &str) -> Self {
        self.fields.push(FormField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: FieldType::Text,
        });
        self
    }

    pub fn number(mut self, key: &str, prompt: &str) -> Self {
        self.fields.push(FormField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: FieldType::Number,
        });
        self
    }

    pub fn boolean(mut self, key: &str, prompt: &str) -> Self {
        self.fields.push(FormField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: FieldType::Boolean,
        });
        self
    }

    pub fn choice(mut self, key: &str, prompt: &str, choices: &[&str]) -> Self {
        self.fields.push(FormField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: FieldType::Choice(choices.iter().map(|s| s.to_string()).collect()),
        });
        self
    }

    pub fn multi_choice(mut self, key: &str, prompt: &str, choices: &[&str]) -> Self {
        self.fields.push(FormField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: FieldType::MultiChoice(choices.iter().map(|s| s.to_string()).collect()),
        });
        self
    }

    pub fn optional(mut self, key: &str, prompt: &str) -> Self {
        self.fields.push(FormField {
            key: key.to_string(),
            prompt: format!("{} (optional)", prompt),
            field_type: FieldType::Optional,
        });
        self
    }

    pub fn validated_text<F>(
        mut self,
        key: &str,
        prompt: &str,
        validator: F,
        error_msg: &str,
    ) -> Self
    where
        F: Fn(&str) -> bool + 'static,
    {
        self.fields.push(FormField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: FieldType::ValidatedText {
                validator: Box::new(validator),
                error_msg: error_msg.to_string(),
            },
        });
        self
    }

    /// Run through all fields and collect the results
    pub fn collect(self) -> HashMap<String, String> {
        let mut results = HashMap::new();

        for field in self.fields {
            let value = match field.field_type {
                FieldType::Text => ask::<String>(&field.prompt),
                FieldType::Number => ask::<f64>(&field.prompt).to_string(),
                FieldType::Boolean => ask::<bool>(&field.prompt).to_string(),
                FieldType::Choice(choices) => {
                    let choice_refs: Vec<&str> = choices.iter().map(|s| s.as_str()).collect();
                    choose(&field.prompt, &choice_refs).to_string()
                }
                FieldType::MultiChoice(choices) => {
                    let choice_refs: Vec<&str> = choices.iter().map(|s| s.as_str()).collect();
                    let selected = multi_select(&field.prompt, &choice_refs);
                    selected.join(", ")
                }
                FieldType::Optional => {
                    let input = ask::<String>(&field.prompt);
                    if input.trim().is_empty() {
                        "".to_string()
                    } else {
                        input
                    }
                }
                FieldType::ValidatedText {
                    validator,
                    error_msg,
                } => {
                    ask_with_validation(&field.prompt, |s: &String| validator(s), Some(&error_msg))
                }
            };

            results.insert(field.key, value);
        }

        results
    }
}

pub fn form() -> Form {
    Form::new()
}
