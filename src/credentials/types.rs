use crate::name::NameParts;

/// Configuration for credential generation.
#[derive(Debug, Clone, Default)]
pub struct CredentialConfig {
    pub generate_email: bool,
    pub generate_password: bool,
    pub email_domain: Option<String>,
}

/// A complete person record with name and optional credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonRecord {
    pub name: NameParts,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl PersonRecord {
    /// Create a new person record from name parts.
    pub fn from_name(name: NameParts) -> Self {
        Self {
            name,
            email: None,
            password: None,
        }
    }

    /// Add an email to the record.
    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// Add a password to the record.
    pub fn with_password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_record_from_name() {
        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };
        let record = PersonRecord::from_name(name.clone());

        assert_eq!(record.name, name);
        assert!(record.email.is_none());
        assert!(record.password.is_none());
    }

    #[test]
    fn test_person_record_with_email() {
        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };
        let record = PersonRecord::from_name(name).with_email("john.smith@example.com".to_string());

        assert_eq!(record.email, Some("john.smith@example.com".to_string()));
    }

    #[test]
    fn test_person_record_with_password() {
        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };
        let record = PersonRecord::from_name(name).with_password("secret123".to_string());

        assert_eq!(record.password, Some("secret123".to_string()));
    }

    #[test]
    fn test_person_record_builder_chain() {
        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };
        let record = PersonRecord::from_name(name)
            .with_email("john@example.com".to_string())
            .with_password("pass123".to_string());

        assert!(record.email.is_some());
        assert!(record.password.is_some());
    }

    #[test]
    fn test_credential_config_default() {
        let config = CredentialConfig::default();

        assert!(!config.generate_email);
        assert!(!config.generate_password);
        assert!(config.email_domain.is_none());
    }
}
