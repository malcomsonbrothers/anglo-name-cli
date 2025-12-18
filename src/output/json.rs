use anyhow::Context;
use serde::Serialize;

use crate::creds::PersonRecord;
use crate::name::NameParts;

use super::format::{NameFormatter, PersonFormatter};

#[derive(Serialize)]
struct JsonName {
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<String>,
    last_name: String,
    full_name: String,
}

impl From<&NameParts> for JsonName {
    fn from(parts: &NameParts) -> Self {
        Self {
            first_name: parts.first_name.clone(),
            middle_name: parts.middle_name.clone(),
            last_name: parts.last_name.clone(),
            full_name: parts.full_name(),
        }
    }
}

#[derive(Serialize)]
struct JsonPerson {
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<String>,
    last_name: String,
    full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birthday: Option<String>,
}

impl From<&PersonRecord> for JsonPerson {
    fn from(record: &PersonRecord) -> Self {
        Self {
            first_name: record.name.first_name.clone(),
            middle_name: record.name.middle_name.clone(),
            last_name: record.name.last_name.clone(),
            full_name: record.name.full_name(),
            email: record.email.clone(),
            password: record.password.clone(),
            birthday: record.birthday.clone(),
        }
    }
}

/// JSON formatter - outputs names as a JSON array.
pub struct JsonFormatter;

impl NameFormatter for JsonFormatter {
    fn format(&self, names: &[NameParts]) -> anyhow::Result<String> {
        let json_names: Vec<JsonName> = names.iter().map(JsonName::from).collect();
        serde_json::to_string_pretty(&json_names).with_context(|| "JsonFormatter::format::0")
    }
}

impl PersonFormatter for JsonFormatter {
    fn format(&self, records: &[PersonRecord]) -> anyhow::Result<String> {
        let json_persons: Vec<JsonPerson> = records.iter().map(JsonPerson::from).collect();
        serde_json::to_string_pretty(&json_persons)
            .with_context(|| "JsonFormatter::format_persons::0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_format_single() {
        let formatter = JsonFormatter;
        let names = vec![NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        }];

        let output = NameFormatter::format(&formatter, &names).unwrap();
        assert!(output.contains("\"first_name\": \"John\""));
        assert!(output.contains("\"last_name\": \"Smith\""));
        assert!(output.contains("\"full_name\": \"John Smith\""));
        assert!(!output.contains("middle_name")); // Skipped when None
    }

    #[test]
    fn test_json_format_with_middle_name() {
        let formatter = JsonFormatter;
        let names = vec![NameParts {
            first_name: "John".to_string(),
            middle_name: Some("William".to_string()),
            last_name: "Smith".to_string(),
        }];

        let output = NameFormatter::format(&formatter, &names).unwrap();
        assert!(output.contains("\"middle_name\": \"William\""));
        assert!(output.contains("\"full_name\": \"John William Smith\""));
    }

    #[test]
    fn test_json_format_is_array() {
        let formatter = JsonFormatter;
        let names = vec![
            NameParts {
                first_name: "John".to_string(),
                middle_name: None,
                last_name: "Smith".to_string(),
            },
            NameParts {
                first_name: "Jane".to_string(),
                middle_name: None,
                last_name: "Doe".to_string(),
            },
        ];

        let output = NameFormatter::format(&formatter, &names).unwrap();
        assert!(output.starts_with('['));
        assert!(output.ends_with(']'));
    }

    #[test]
    fn test_json_format_person_without_credentials() {
        let formatter = JsonFormatter;
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
        assert!(output.contains("\"first_name\": \"John\""));
        assert!(!output.contains("email"));
        assert!(!output.contains("password"));
    }

    #[test]
    fn test_json_format_person_with_email() {
        let formatter = JsonFormatter;
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
        assert!(output.contains("\"email\": \"john.smith@example.com\""));
        assert!(!output.contains("password"));
    }

    #[test]
    fn test_json_format_person_with_both() {
        let formatter = JsonFormatter;
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
        assert!(output.contains("\"email\": \"john.smith@example.com\""));
        assert!(output.contains("\"password\": \"secret123\""));
    }
}
