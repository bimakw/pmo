use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::shared::DomainError;

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| DomainError::InternalError("Failed to hash password".into()))
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| DomainError::InternalError("Invalid password hash".into()))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ Hash Password Tests ============

    #[test]
    fn test_hash_password_returns_valid_hash() {
        let password = "TestPassword123!";
        let hash = AuthService::hash_password(password);
        assert!(hash.is_ok());
        // Argon2 hash starts with $argon2
        assert!(hash.unwrap().starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_different_hashes_for_same_password() {
        let password = "TestPassword123!";
        let hash1 = AuthService::hash_password(password).unwrap();
        let hash2 = AuthService::hash_password(password).unwrap();
        // Karena salt berbeda, hash harus berbeda
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_password_empty_password() {
        // Empty password still produces a hash
        let hash = AuthService::hash_password("");
        assert!(hash.is_ok());
    }

    #[test]
    fn test_hash_password_long_password() {
        let long_password = "a".repeat(200);
        let hash = AuthService::hash_password(&long_password);
        assert!(hash.is_ok());
    }

    #[test]
    fn test_hash_password_unicode() {
        let unicode_pass = "パスワード123!Abc";
        let hash = AuthService::hash_password(unicode_pass);
        assert!(hash.is_ok());
    }

    // ============ Verify Password Tests ============

    #[test]
    fn test_verify_password_correct() {
        let password = "TestPassword123!";
        let hash = AuthService::hash_password(password).unwrap();
        let result = AuthService::verify_password(password, &hash);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_password_wrong_password() {
        let password = "TestPassword123!";
        let wrong_password = "WrongPassword456!";
        let hash = AuthService::hash_password(password).unwrap();
        let result = AuthService::verify_password(wrong_password, &hash);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_password_similar_wrong_password() {
        let password = "TestPassword123!";
        let similar = "TestPassword123"; // missing !
        let hash = AuthService::hash_password(password).unwrap();
        let result = AuthService::verify_password(similar, &hash);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_password_case_sensitive() {
        let password = "TestPassword123!";
        let wrong_case = "testpassword123!";
        let hash = AuthService::hash_password(password).unwrap();
        let result = AuthService::verify_password(wrong_case, &hash);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_password_invalid_hash_format() {
        let result = AuthService::verify_password("password", "not-a-valid-hash");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_password_empty_hash() {
        let result = AuthService::verify_password("password", "");
        assert!(result.is_err());
    }

    // ============ Integration Tests ============

    #[test]
    fn test_hash_then_verify_multiple_passwords() {
        let passwords = vec![
            "Simple123!",
            "Complex@Password#2024",
            "Unicode日本語Abc1!",
            "  spaces  Pass1! ",
            "Short1!a",
        ];

        for password in passwords {
            let hash = AuthService::hash_password(password).unwrap();
            let verified = AuthService::verify_password(password, &hash).unwrap();
            assert!(verified, "Failed to verify password: {}", password);
        }
    }

    #[test]
    fn test_hash_format_contains_required_parts() {
        let hash = AuthService::hash_password("Test123!").unwrap();
        // Argon2 hash format: $argon2id$v=VERSION$m=MEMORY,t=TIME,p=PARALLELISM$SALT$HASH
        let parts: Vec<&str> = hash.split('$').collect();
        assert!(parts.len() >= 5, "Hash should have proper argon2 format");
    }
}
