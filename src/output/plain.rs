use crate::creds::PersonRecord;
use crate::name::NameParts;

use super::format::{NameFormatter, PersonFormatter};

/// Plain text formatter - outputs one name per line.
pub struct PlainFormatter;

impl NameFormatter for PlainFormatter {
    fn format(&self, names: &[NameParts]) -> anyhow::Result<String> {
        Ok(names
            .iter()
            .map(|n| n.full_name())
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

impl PersonFormatter for PlainFormatter {
    fn format(&self, records: &[PersonRecord]) -> anyhow::Result<String> {
        Ok(records
            .iter()
            .map(|r| {
                let mut parts = vec![r.name.full_name()];
                if let Some(ref email) = r.email {
                    parts.push(email.clone());
                }
                if let Some(ref password) = r.password {
                    parts.push(password.clone());
                }
                if let Some(ref birthday) = r.birthday {
                    parts.push(birthday.clone());
                }
                parts.join(" | ")
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_format_single() {
        let formatter = PlainFormatter;
        let names = vec![NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        }];

        let output = NameFormatter::format(&formatter, &names).unwrap();
        assert_eq!(output, "John Smith");
    }

    #[test]
    fn test_plain_format_multiple() {
        let formatter = PlainFormatter;
        let names = vec![
            NameParts {
                first_name: "John".to_string(),
                middle_name: None,
                last_name: "Smith".to_string(),
            },
            NameParts {
                first_name: "Jane".to_string(),
                middle_name: Some("Marie".to_string()),
                last_name: "Doe".to_string(),
            },
        ];

        let output = NameFormatter::format(&formatter, &names).unwrap();
        assert_eq!(output, "John Smith\nJane Marie Doe");
    }

    #[test]
    fn test_plain_format_empty() {
        let formatter = PlainFormatter;
        let names: Vec<NameParts> = vec![];

        let output = NameFormatter::format(&formatter, &names).unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn test_plain_format_person_record_name_only() {
        let formatter = PlainFormatter;
        let records = vec![PersonRecord {
            name: NameParts {
                first_name: "John".to_string(),
                middle_name: None,
                last_name: "Smith".to_string(),
            },
            email: None,
            password: None,
            birthday: None,
        }];

        let output = PersonFormatter::format(&formatter, &records).unwrap();
        assert_eq!(output, "John Smith");
    }

    #[test]
    fn test_plain_format_person_record_with_email() {
        let formatter = PlainFormatter;
        let records = vec![PersonRecord {
            name: NameParts {
                first_name: "John".to_string(),
                middle_name: None,
                last_name: "Smith".to_string(),
            },
            email: Some("john.smith@example.com".to_string()),
            password: None,
            birthday: None,
        }];

        let output = PersonFormatter::format(&formatter, &records).unwrap();
        assert_eq!(output, "John Smith | john.smith@example.com");
    }

    #[test]
    fn test_plain_format_person_record_with_password() {
        let formatter = PlainFormatter;
        let records = vec![PersonRecord {
            name: NameParts {
                first_name: "John".to_string(),
                middle_name: None,
                last_name: "Smith".to_string(),
            },
            email: None,
            password: Some("secret123".to_string()),
            birthday: None,
        }];

        let output = PersonFormatter::format(&formatter, &records).unwrap();
        assert_eq!(output, "John Smith | secret123");
    }

    #[test]
    fn test_plain_format_person_record_with_both() {
        let formatter = PlainFormatter;
        let records = vec![PersonRecord {
            name: NameParts {
                first_name: "John".to_string(),
                middle_name: None,
                last_name: "Smith".to_string(),
            },
            email: Some("john.smith@example.com".to_string()),
            password: Some("secret123".to_string()),
            birthday: None,
        }];

        let output = PersonFormatter::format(&formatter, &records).unwrap();
        assert_eq!(output, "John Smith | john.smith@example.com | secret123");
    }
}
