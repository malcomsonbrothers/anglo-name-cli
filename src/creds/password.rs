use rand::seq::SliceRandom;
use rand::Rng;

/// Trait for password generation strategies.
pub trait PasswordGenerator: Send + Sync {
    /// Generate a random password.
    fn generate(&self) -> String;
}

/// Standard password generator with configurable character sets.
/// Uses a cryptographically secure random number generator.
pub struct StandardPasswordGenerator {
    length: usize,
}

impl StandardPasswordGenerator {
    /// Create a new standard password generator with default length (32).
    pub fn new() -> Self {
        Self { length: 32 }
    }
}

impl Default for StandardPasswordGenerator {
    fn default() -> Self {
        Self::new()
    }
}

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"!@#$%^&*";

impl PasswordGenerator for StandardPasswordGenerator {
    fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        let all_chars: Vec<u8> = [UPPERCASE, LOWERCASE, DIGITS, SPECIAL].concat();

        let mut password = Vec::with_capacity(self.length);

        // Guarantee at least one character from each category
        password.push(UPPERCASE[rng.gen_range(0..UPPERCASE.len())]);
        password.push(LOWERCASE[rng.gen_range(0..LOWERCASE.len())]);
        password.push(DIGITS[rng.gen_range(0..DIGITS.len())]);
        password.push(SPECIAL[rng.gen_range(0..SPECIAL.len())]);

        // Fill remaining characters randomly from combined pool
        let remaining = self.length.saturating_sub(4);
        for _ in 0..remaining {
            password.push(all_chars[rng.gen_range(0..all_chars.len())]);
        }

        // Shuffle to avoid predictable positions
        password.shuffle(&mut rng);

        // Safe because all characters are ASCII
        String::from_utf8(password).expect("password contains only ASCII characters")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        let generator = StandardPasswordGenerator::new();
        let password = generator.generate();

        assert_eq!(password.len(), 32);
    }

    #[test]
    fn test_password_contains_uppercase() {
        let generator = StandardPasswordGenerator::new();
        let password = generator.generate();

        assert!(
            password.chars().any(|c| c.is_ascii_uppercase()),
            "Password should contain at least one uppercase letter"
        );
    }

    #[test]
    fn test_password_contains_lowercase() {
        let generator = StandardPasswordGenerator::new();
        let password = generator.generate();

        assert!(
            password.chars().any(|c| c.is_ascii_lowercase()),
            "Password should contain at least one lowercase letter"
        );
    }

    #[test]
    fn test_password_contains_digit() {
        let generator = StandardPasswordGenerator::new();
        let password = generator.generate();

        assert!(
            password.chars().any(|c| c.is_ascii_digit()),
            "Password should contain at least one digit"
        );
    }

    #[test]
    fn test_password_contains_special() {
        let generator = StandardPasswordGenerator::new();
        let password = generator.generate();
        let special_chars = "!@#$%^&*";

        assert!(
            password.chars().any(|c| special_chars.contains(c)),
            "Password should contain at least one special character"
        );
    }

    #[test]
    fn test_password_only_contains_expected_characters() {
        let generator = StandardPasswordGenerator::new();
        let password = generator.generate();
        let valid_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";

        for c in password.chars() {
            assert!(
                valid_chars.contains(c),
                "Password contains unexpected character: {}",
                c
            );
        }
    }

    #[test]
    fn test_passwords_are_different() {
        let generator = StandardPasswordGenerator::new();
        let password1 = generator.generate();
        let password2 = generator.generate();

        // While theoretically possible to generate the same password twice,
        // it's extremely unlikely with 32 characters from 70+ character set
        assert_ne!(
            password1, password2,
            "Two generated passwords should be different"
        );
    }
}
