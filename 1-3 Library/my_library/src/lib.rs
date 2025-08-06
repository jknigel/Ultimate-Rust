//! # String Utilities
//!
//! `str_utils` is a collection of utilities to make working with strings more convenient.
//! This library aims to provide simple and efficient functions for common string manipulations.

/// Reverses a given string slice.
///
/// This function takes a string slice and returns a new `String`
/// with its characters in reverse order.
///
/// # Examples
///
/// ```
/// let reversed = my_library::reverse("Hello");
/// assert_eq!(reversed, "olleH");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Checks if a string is a palindrome.
///
/// A palindrome is a word, phrase, number, or other sequence of characters
/// which reads the same backward as forward. This check is case-insensitive.
///
/// # Examples
///
/// ```
/// assert!(my_library::is_palindrome("level"));
/// assert!(!my_library::is_palindrome("hello"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed = reverse(&cleaned);
    cleaned == reversed
}

// This is the tests module
#[cfg(test)]
mod tests {
    use super::*; // Import functions from the parent module (our library)

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("rust"), "tsur");
        assert_eq!(reverse("level"), "level");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("Was it a car or a cat I saw?"));
    }
}