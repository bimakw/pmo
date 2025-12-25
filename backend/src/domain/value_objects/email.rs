use serde::{Deserialize, Serialize};
use std::fmt;

use crate::shared::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, DomainError> {
        let email = email.into();

        if email.is_empty() {
            return Err(DomainError::ValidationError("Email cannot be empty".into()));
        }

        if !email.contains('@') || !email.contains('.') {
            return Err(DomainError::ValidationError("Invalid email format".into()));
        }

        Ok(Self(email.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ Valid Email Tests ============

    #[test]
    fn test_valid_simple_email() {
        let email = Email::new("test@example.com");
        assert!(email.is_ok());
        assert_eq!(email.unwrap().as_str(), "test@example.com");
    }

    #[test]
    fn test_valid_email_with_subdomain() {
        let email = Email::new("user@mail.example.com");
        assert!(email.is_ok());
    }

    #[test]
    fn test_valid_email_with_plus() {
        let email = Email::new("user+tag@example.com");
        assert!(email.is_ok());
    }

    #[test]
    fn test_valid_email_with_dots_in_local() {
        let email = Email::new("first.last@example.com");
        assert!(email.is_ok());
    }

    #[test]
    fn test_valid_email_with_numbers() {
        let email = Email::new("user123@example456.com");
        assert!(email.is_ok());
    }

    // ============ Invalid Email Tests ============

    #[test]
    fn test_empty_email_fails() {
        let email = Email::new("");
        assert!(email.is_err());
    }

    #[test]
    fn test_no_at_symbol_fails() {
        let email = Email::new("testexample.com");
        assert!(email.is_err());
    }

    #[test]
    fn test_no_dot_fails() {
        let email = Email::new("test@examplecom");
        assert!(email.is_err());
    }

    #[test]
    fn test_only_at_symbol_fails() {
        let email = Email::new("@");
        assert!(email.is_err());
    }

    #[test]
    fn test_only_dot_fails() {
        let email = Email::new(".");
        assert!(email.is_err());
    }

    // ============ Normalization Tests ============

    #[test]
    fn test_email_converted_to_lowercase() {
        let email = Email::new("TEST@EXAMPLE.COM").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn test_mixed_case_normalized() {
        let email = Email::new("TeSt@ExAmPlE.CoM").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    // ============ Display & AsRef Tests ============

    #[test]
    fn test_display_shows_email() {
        let email = Email::new("test@example.com").unwrap();
        assert_eq!(format!("{}", email), "test@example.com");
    }

    #[test]
    fn test_as_ref_returns_str() {
        let email = Email::new("test@example.com").unwrap();
        let reference: &str = email.as_ref();
        assert_eq!(reference, "test@example.com");
    }

    // ============ Clone & PartialEq Tests ============

    #[test]
    fn test_email_cloneable() {
        let email1 = Email::new("test@example.com").unwrap();
        let email2 = email1.clone();
        assert_eq!(email1, email2);
    }

    #[test]
    fn test_same_emails_are_equal() {
        let email1 = Email::new("test@example.com").unwrap();
        let email2 = Email::new("test@example.com").unwrap();
        assert_eq!(email1, email2);
    }

    #[test]
    fn test_different_emails_not_equal() {
        let email1 = Email::new("test1@example.com").unwrap();
        let email2 = Email::new("test2@example.com").unwrap();
        assert_ne!(email1, email2);
    }

    #[test]
    fn test_case_different_emails_equal_after_normalize() {
        let email1 = Email::new("TEST@example.com").unwrap();
        let email2 = Email::new("test@example.com").unwrap();
        assert_eq!(email1, email2);
    }
}
