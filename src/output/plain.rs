use crate::name::NameParts;

use super::format::NameFormatter;

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

        let output = formatter.format(&names).unwrap();
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

        let output = formatter.format(&names).unwrap();
        assert_eq!(output, "John Smith\nJane Marie Doe");
    }

    #[test]
    fn test_plain_format_empty() {
        let formatter = PlainFormatter;
        let names: Vec<NameParts> = vec![];

        let output = formatter.format(&names).unwrap();
        assert_eq!(output, "");
    }
}
