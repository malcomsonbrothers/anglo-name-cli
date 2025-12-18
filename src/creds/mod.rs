mod domain;
mod email;
mod password;
mod types;

pub use domain::{DefaultDomainSource, DomainSource, FixedDomainSource};
pub use email::{EmailGenerator, StandardEmailGenerator};
pub use password::{PasswordGenerator, StandardPasswordGenerator};
pub use types::{CredentialConfig, PersonRecord};

use crate::birthday::{BirthdayGenerator, DateRange};
use crate::name::NameParts;

/// Enriches name records with credentials (email, password, and/or birthday).
pub struct CredentialEnricher {
    email_generator: Option<Box<dyn EmailGenerator>>,
    password_generator: Option<Box<dyn PasswordGenerator>>,
    domain_source: Box<dyn DomainSource>,
    birthday_range: Option<DateRange>,
}

impl CredentialEnricher {
    /// Create a new credential enricher from configuration.
    pub fn new(config: &CredentialConfig) -> Self {
        let email_generator: Option<Box<dyn EmailGenerator>> = if config.generate_email {
            Some(Box::new(StandardEmailGenerator))
        } else {
            None
        };

        let password_generator: Option<Box<dyn PasswordGenerator>> = if config.generate_password {
            Some(Box::new(StandardPasswordGenerator::new()))
        } else {
            None
        };

        let domain_source: Box<dyn DomainSource> = match &config.email_domain {
            Some(domain) => Box::new(FixedDomainSource::new(domain.clone())),
            None => Box::new(DefaultDomainSource),
        };

        Self {
            email_generator,
            password_generator,
            domain_source,
            birthday_range: config.birthday_range.clone(),
        }
    }

    /// Enrich a single name with credentials.
    pub fn enrich(&self, name: NameParts) -> PersonRecord {
        // Generate email first (needs reference to name)
        let email = if let Some(ref email_gen) = self.email_generator {
            let domain = self.domain_source.select_domain();
            Some(email_gen.generate(&name, domain))
        } else {
            None
        };

        // Generate password
        let password = self
            .password_generator
            .as_ref()
            .map(|password_gen| password_gen.generate());

        // Generate birthday
        let birthday = self
            .birthday_range
            .as_ref()
            .map(|range| BirthdayGenerator.generate(range));

        PersonRecord {
            name,
            email,
            password,
            birthday,
        }
    }

    /// Enrich multiple names with credentials.
    pub fn enrich_many(&self, names: Vec<NameParts>) -> Vec<PersonRecord> {
        names.into_iter().map(|n| self.enrich(n)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enricher_no_credentials() {
        let config = CredentialConfig::default();
        let enricher = CredentialEnricher::new(&config);

        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };

        let record = enricher.enrich(name);

        assert!(record.email.is_none());
        assert!(record.password.is_none());
    }

    #[test]
    fn test_enricher_with_email() {
        let config = CredentialConfig {
            generate_email: true,
            generate_password: false,
            email_domain: Some("example.com".to_string()),
            birthday_range: None,
        };
        let enricher = CredentialEnricher::new(&config);

        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };

        let record = enricher.enrich(name);

        assert_eq!(record.email, Some("john.smith@example.com".to_string()));
        assert!(record.password.is_none());
    }

    #[test]
    fn test_enricher_with_password() {
        let config = CredentialConfig {
            generate_email: false,
            generate_password: true,
            email_domain: None,
            birthday_range: None,
        };
        let enricher = CredentialEnricher::new(&config);

        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };

        let record = enricher.enrich(name);

        assert!(record.email.is_none());
        assert!(record.password.is_some());
        assert_eq!(record.password.as_ref().unwrap().len(), 32);
    }

    #[test]
    fn test_enricher_with_both() {
        let config = CredentialConfig {
            generate_email: true,
            generate_password: true,
            email_domain: Some("test.org".to_string()),
            birthday_range: None,
        };
        let enricher = CredentialEnricher::new(&config);

        let name = NameParts {
            first_name: "Jane".to_string(),
            middle_name: Some("Marie".to_string()),
            last_name: "Doe".to_string(),
        };

        let record = enricher.enrich(name);

        assert_eq!(record.email, Some("jane.marie.doe@test.org".to_string()));
        assert!(record.password.is_some());
        assert_eq!(record.password.as_ref().unwrap().len(), 32);
    }

    #[test]
    fn test_enricher_many() {
        let config = CredentialConfig {
            generate_email: true,
            generate_password: false,
            email_domain: Some("bulk.com".to_string()),
            birthday_range: None,
        };
        let enricher = CredentialEnricher::new(&config);

        let names = vec![
            NameParts {
                first_name: "Alice".to_string(),
                middle_name: None,
                last_name: "Brown".to_string(),
            },
            NameParts {
                first_name: "Bob".to_string(),
                middle_name: None,
                last_name: "White".to_string(),
            },
        ];

        let records = enricher.enrich_many(names);

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].email, Some("alice.brown@bulk.com".to_string()));
        assert_eq!(records[1].email, Some("bob.white@bulk.com".to_string()));
    }
}
