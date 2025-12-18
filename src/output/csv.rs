use anyhow::Context;

use crate::creds::PersonRecord;
use crate::name::NameParts;

use super::format::{NameFormatter, PersonFormatter};

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

impl PersonFormatter for CsvFormatter {
    fn format(&self, records: &[PersonRecord]) -> anyhow::Result<String> {
        let mut wtr = csv::Writer::from_writer(Vec::new());

        // Determine which optional columns to include based on first record
        let has_email = records.first().is_some_and(|r| r.email.is_some());
        let has_password = records.first().is_some_and(|r| r.password.is_some());
        let has_birthday = records.first().is_some_and(|r| r.birthday.is_some());

        // Build header dynamically
        let mut header = vec!["first_name", "middle_name", "last_name", "full_name"];
        if has_email {
            header.push("email");
        }
        if has_password {
            header.push("password");
        }
        if has_birthday {
            header.push("birthday");
        }
        wtr.write_record(&header)
            .with_context(|| "CsvFormatter::format_persons::0")?;

        for (i, record) in records.iter().enumerate() {
            let mut row = vec![
                record.name.first_name.clone(),
                record.name.middle_name.clone().unwrap_or_default(),
                record.name.last_name.clone(),
                record.name.full_name(),
            ];
            if has_email {
                row.push(record.email.clone().unwrap_or_default());
            }
            if has_password {
                row.push(record.password.clone().unwrap_or_default());
            }
            if has_birthday {
                row.push(record.birthday.clone().unwrap_or_default());
            }
            wtr.write_record(&row)
                .with_context(|| format!("CsvFormatter::format_persons::{}", i + 1))?;
        }

        let bytes = wtr
            .into_inner()
            .with_context(|| "CsvFormatter::format_persons::flush")?;

        String::from_utf8(bytes).with_context(|| "CsvFormatter::format_persons::utf8")
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

        let output = NameFormatter::format(&formatter, &names).unwrap();
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

        let output = NameFormatter::format(&formatter, &names).unwrap();
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

        let output = NameFormatter::format(&formatter, &names).unwrap();
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

        let output = NameFormatter::format(&formatter, &names).unwrap();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3); // Header + 2 data rows
    }

    #[test]
    fn test_csv_format_person_without_credentials() {
        let formatter = CsvFormatter;
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
        assert!(output.starts_with("first_name,middle_name,last_name,full_name\n"));
        assert!(!output.contains("email"));
        assert!(!output.contains("password"));
    }

    #[test]
    fn test_csv_format_person_with_email() {
        let formatter = CsvFormatter;
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
        assert!(output.starts_with("first_name,middle_name,last_name,full_name,email\n"));
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[1], "John,,Smith,John Smith,john.smith@example.com");
    }

    #[test]
    fn test_csv_format_person_with_both() {
        let formatter = CsvFormatter;
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
        assert!(output.starts_with("first_name,middle_name,last_name,full_name,email,password\n"));
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(
            lines[1],
            "John,,Smith,John Smith,john.smith@example.com,secret123"
        );
    }
}
