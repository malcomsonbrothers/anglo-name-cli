use crate::name::NameParts;

/// Trait for email generation strategies.
pub trait EmailGenerator: Send + Sync {
    /// Generate an email address from name parts and domain.
    fn generate(&self, name: &NameParts, domain: &str) -> String;
}

/// Standard email generator using lowercase names with dots.
pub struct StandardEmailGenerator;

impl EmailGenerator for StandardEmailGenerator {
    fn generate(&self, name: &NameParts, domain: &str) -> String {
        let local_part = build_local_part(name);
        format!("{}@{}", local_part, domain)
    }
}

/// Build the local part of an email from name parts.
fn build_local_part(name: &NameParts) -> String {
    let mut parts = vec![sanitise_for_email(&name.first_name)];

    if let Some(ref middle) = name.middle_name {
        parts.push(sanitise_for_email(middle));
    }

    parts.push(sanitise_for_email(&name.last_name));

    parts.join(".")
}

/// Sanitise a string for use in an email local part.
/// - Keeps only ASCII alphabetic characters
/// - Converts to lowercase
fn sanitise_for_email(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitise_removes_apostrophes() {
        assert_eq!(sanitise_for_email("O'Brien"), "obrien");
    }

    #[test]
    fn test_sanitise_removes_hyphens() {
        assert_eq!(sanitise_for_email("Mary-Jane"), "maryjane");
    }

    #[test]
    fn test_sanitise_removes_spaces() {
        assert_eq!(sanitise_for_email("De La Cruz"), "delacruz");
    }

    #[test]
    fn test_sanitise_lowercases() {
        assert_eq!(sanitise_for_email("McDonald"), "mcdonald");
    }

    #[test]
    fn test_sanitise_simple_name() {
        assert_eq!(sanitise_for_email("John"), "john");
    }

    #[test]
    fn test_email_format_without_middle() {
        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };
        let generator = StandardEmailGenerator;

        assert_eq!(
            generator.generate(&name, "example.com"),
            "john.smith@example.com"
        );
    }

    #[test]
    fn test_email_format_with_middle() {
        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: Some("William".to_string()),
            last_name: "Smith".to_string(),
        };
        let generator = StandardEmailGenerator;

        assert_eq!(
            generator.generate(&name, "example.com"),
            "john.william.smith@example.com"
        );
    }

    #[test]
    fn test_email_with_apostrophe_in_name() {
        let name = NameParts {
            first_name: "Sean".to_string(),
            middle_name: None,
            last_name: "O'Brien".to_string(),
        };
        let generator = StandardEmailGenerator;

        assert_eq!(
            generator.generate(&name, "gmail.com"),
            "sean.obrien@gmail.com"
        );
    }

    #[test]
    fn test_email_with_hyphenated_name() {
        let name = NameParts {
            first_name: "Mary-Jane".to_string(),
            middle_name: None,
            last_name: "Watson".to_string(),
        };
        let generator = StandardEmailGenerator;

        assert_eq!(
            generator.generate(&name, "outlook.com"),
            "maryjane.watson@outlook.com"
        );
    }
}
