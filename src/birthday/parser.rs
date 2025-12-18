use anyhow::{bail, Context};
use chrono::{Days, Local, Months, NaiveDate};

use super::types::DateRange;

/// Parse a date string in DD-MM-YYYY format into a NaiveDate.
pub fn parse_date(input: &str) -> anyhow::Result<NaiveDate> {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 3 {
        bail!(
            "Invalid date format '{}'. Expected DD-MM-YYYY (e.g., 01-01-1990)",
            input
        );
    }

    let day: u32 = parts[0]
        .parse()
        .with_context(|| format!("parse_date::0 - invalid day '{}' in '{}'", parts[0], input))?;

    let month: u32 = parts[1].parse().with_context(|| {
        format!(
            "parse_date::1 - invalid month '{}' in '{}'",
            parts[1], input
        )
    })?;

    let year: i32 = parts[2]
        .parse()
        .with_context(|| format!("parse_date::2 - invalid year '{}' in '{}'", parts[2], input))?;

    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid date '{}'. Day {}, month {}, year {} do not form a valid date",
            input,
            day,
            month,
            year
        )
    })
}

/// Create a DateRange from date strings, validating that from <= to.
pub fn parse_date_range(from: &str, to: &str) -> anyhow::Result<DateRange> {
    let from_date = parse_date(from).with_context(|| "parse_date_range::0")?;

    let to_date = parse_date(to).with_context(|| "parse_date_range::1")?;

    if from_date > to_date {
        bail!(
            "Invalid date range: --birthday-from ({}) must be on or before --birthday-to ({})",
            from,
            to
        );
    }

    Ok(DateRange {
        from: from_date,
        to: to_date,
    })
}

/// Convert a single age to a DateRange.
///
/// For a person who is exactly N years old today:
/// - Latest birthday: They just turned N today (today - N years)
/// - Earliest birthday: They are about to turn N+1 tomorrow (today - (N+1) years + 1 day)
pub fn age_to_date_range(age: u32) -> DateRange {
    let today = Local::now().date_naive();

    // Latest: just turned `age` today
    let to = today
        .checked_sub_months(Months::new(age * 12))
        .expect("valid date subtraction");

    // Earliest: about to turn `age+1` tomorrow
    let from = today
        .checked_sub_months(Months::new((age + 1) * 12))
        .expect("valid date subtraction")
        .checked_add_days(Days::new(1))
        .expect("valid date addition");

    DateRange { from, to }
}

/// Convert an age range to a DateRange.
///
/// For people between min_age and max_age years old today:
/// - Oldest person (max_age) is about to turn max_age+1 tomorrow
/// - Youngest person (min_age) just turned min_age today
pub fn age_range_to_date_range(min_age: u32, max_age: u32) -> anyhow::Result<DateRange> {
    if min_age > max_age {
        bail!("--age-from ({}) must be <= --age-to ({})", min_age, max_age);
    }

    let today = Local::now().date_naive();

    // Oldest person (max_age) about to turn max_age+1 tomorrow
    let from = today
        .checked_sub_months(Months::new((max_age + 1) * 12))
        .expect("valid date subtraction")
        .checked_add_days(Days::new(1))
        .expect("valid date addition");

    // Youngest person (min_age) just turned min_age today
    let to = today
        .checked_sub_months(Months::new(min_age * 12))
        .expect("valid date subtraction");

    Ok(DateRange { from, to })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_date() {
        let date = parse_date("15-06-1995").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(1995, 6, 15).unwrap());
    }

    #[test]
    fn test_parse_valid_date_leading_zeros() {
        let date = parse_date("01-01-2000").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    }

    #[test]
    fn test_parse_invalid_date_feb_31() {
        let result = parse_date("31-02-1990");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid date"));
    }

    #[test]
    fn test_parse_leap_year_valid() {
        let date = parse_date("29-02-2024").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
    }

    #[test]
    fn test_parse_leap_year_invalid() {
        let result = parse_date("29-02-2023");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_month() {
        let result = parse_date("01-13-1990");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_day_zero() {
        let result = parse_date("00-01-1990");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_wrong_format_iso() {
        // ISO format (YYYY-MM-DD) should fail since we expect DD-MM-YYYY
        let result = parse_date("1990-01-01");
        assert!(result.is_err());
        // Will fail because day=1990 is invalid
        assert!(result.unwrap_err().to_string().contains("Invalid date"));
    }

    #[test]
    fn test_parse_wrong_format_too_few_parts() {
        let result = parse_date("01-01");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("DD-MM-YYYY"));
    }

    #[test]
    fn test_date_range_valid() {
        let range = parse_date_range("01-01-2000", "31-12-2000").unwrap();
        assert_eq!(range.from, NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
        assert_eq!(range.to, NaiveDate::from_ymd_opt(2000, 12, 31).unwrap());
    }

    #[test]
    fn test_date_range_same_date() {
        let range = parse_date_range("15-06-1995", "15-06-1995").unwrap();
        assert_eq!(range.from, range.to);
    }

    #[test]
    fn test_date_range_from_after_to() {
        let result = parse_date_range("31-12-2000", "01-01-2000");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must be on or before"));
    }

    #[test]
    fn test_age_to_date_range() {
        let range = age_to_date_range(25);
        // The range should span exactly one year
        assert!(range.from < range.to);
    }

    #[test]
    fn test_age_range_to_date_range() {
        let range = age_range_to_date_range(25, 30).unwrap();
        // Oldest person's birthday should be earlier than youngest
        assert!(range.from < range.to);
    }

    #[test]
    fn test_age_range_same_age() {
        let range = age_range_to_date_range(25, 25).unwrap();
        // Should be equivalent to single age
        let single = age_to_date_range(25);
        assert_eq!(range.from, single.from);
        assert_eq!(range.to, single.to);
    }

    #[test]
    fn test_age_range_invalid_min_greater_max() {
        let result = age_range_to_date_range(30, 25);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be <="));
    }
}
