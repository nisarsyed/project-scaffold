//! {{description}}
//!
//! # Example
//!
//! ```
//! use {{project_name}}::add;
//!
//! let result = add(2, 3);
//! assert_eq!(result, 5);
//! ```

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = {{project_name}}::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second number from the first.
///
/// # Examples
///
/// ```
/// let result = {{project_name}}::subtract(5, 3);
/// assert_eq!(result, 2);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(1, 1), 0);
        assert_eq!(subtract(0, 5), -5);
    }
}
