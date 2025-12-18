use anyhow::Context;
use serde::Serialize;

use crate::name::NameParts;

use super::format::NameFormatter;

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

/// JSON formatter - outputs names as a JSON array.
pub struct JsonFormatter;

impl NameFormatter for JsonFormatter {
    fn format(&self, names: &[NameParts]) -> anyhow::Result<String> {
        let json_names: Vec<JsonName> = names.iter().map(JsonName::from).collect();
        serde_json::to_string_pretty(&json_names).with_context(|| "JsonFormatter::format::0")
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

        let output = formatter.format(&names).unwrap();
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

        let output = formatter.format(&names).unwrap();
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

        let output = formatter.format(&names).unwrap();
        assert!(output.starts_with('['));
        assert!(output.ends_with(']'));
    }
}
