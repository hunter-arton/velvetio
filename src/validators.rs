// src/validators.rs

/// String is not empty after trimming
pub fn not_empty(s: &String) -> bool {
    !s.trim().is_empty()
}

/// String has at least min characters
pub fn min_length(min: usize) -> impl Fn(&String) -> bool {
    move |s: &String| s.len() >= min
}

/// String has at most max characters
pub fn max_length(max: usize) -> impl Fn(&String) -> bool {
    move |s: &String| s.len() <= max
}

/// Number is positive (> 0)
pub fn is_positive<T: PartialOrd + Default>(n: &T) -> bool {
    *n > T::default()
}

/// Number is within range (inclusive)
pub fn in_range<T: PartialOrd + Copy>(min: T, max: T) -> impl Fn(&T) -> bool {
    move |n: &T| *n >= min && *n <= max
}

/// Both validators must pass
pub fn and<T, F1, F2>(validator1: F1, validator2: F2) -> impl Fn(&T) -> bool
where
    F1: Fn(&T) -> bool,
    F2: Fn(&T) -> bool,
{
    move |value: &T| validator1(value) && validator2(value)
}

/// Either validator can pass
pub fn or<T, F1, F2>(validator1: F1, validator2: F2) -> impl Fn(&T) -> bool
where
    F1: Fn(&T) -> bool,
    F2: Fn(&T) -> bool,
{
    move |value: &T| validator1(value) || validator2(value)
}

// Custom validator examples:
//
// Email: |s: &String| s.contains('@') && s.contains('.')
// Strong password: |s: &String| s.len() >= 8 && s.chars().any(|c| c.is_uppercase())
// Valid port: |p: &u16| *p >= 1024 && *p <= 65535
