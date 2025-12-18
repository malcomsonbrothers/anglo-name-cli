use anyhow::Context;

use crate::name::NameParts;

use super::format::NameFormatter;

/// CSV formatter - outputs names as CSV with headers.
pub struct CsvFormatter;

impl NameFormatter for CsvFormatter {
    fn format(&self, names: &[NameParts]) -> anyhow::Result<String> {
        let mut wtr = csv::Writer::from_writer(Vec::new());

        wtr.write_record(["first_name", "middle_name", "last_name", "full_name"])
            .with_context(|| "CsvFormatter::format::0")?;

        for (i, name) in names.iter().enumerate() {
            wtr.write_record([
                &name.first_name,
                name.middle_name.as_deref().unwrap_or(""),
                &name.last_name,
                &name.full_name(),
            ])
            .with_context(|| format!("CsvFormatter::format::{}", i + 1))?;
        }

        let bytes = wtr
            .into_inner()
            .with_context(|| "CsvFormatter::format::flush")?;

        String::from_utf8(bytes).with_context(|| "CsvFormatter::format::utf8")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_format_header() {
        let formatter = CsvFormatter;
        let names = vec![NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        }];

        let output = formatter.format(&names).unwrap();
        assert!(output.starts_with("first_name,middle_name,last_name,full_name"));
    }

    #[test]
    fn test_csv_format_data() {
        let formatter = CsvFormatter;
        let names = vec![NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        }];

        let output = formatter.format(&names).unwrap();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[1], "John,,Smith,John Smith");
    }

    #[test]
    fn test_csv_format_with_middle_name() {
        let formatter = CsvFormatter;
        let names = vec![NameParts {
            first_name: "John".to_string(),
            middle_name: Some("William".to_string()),
            last_name: "Smith".to_string(),
        }];

        let output = formatter.format(&names).unwrap();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[1], "John,William,Smith,John William Smith");
    }

    #[test]
    fn test_csv_format_multiple() {
        let formatter = CsvFormatter;
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
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3); // Header + 2 data rows
    }
}
