mod parser;
mod types;

pub use parser::{age_range_to_date_range, age_to_date_range, parse_date_range};
pub use types::DateRange;

pub use self::generator::BirthdayGenerator;

mod generator {
    use chrono::{Datelike, NaiveDate};

    use super::DateRange;

    /// Generates random birthdays within a date range.
    pub struct BirthdayGenerator;

    impl BirthdayGenerator {
        /// Generate a random birthday within the given date range.
        /// Uses chrono's num_days_from_ce() for uniform distribution.
        pub fn generate(&self, range: &DateRange) -> String {
            let from_days = range.from.num_days_from_ce();
            let to_days = range.to.num_days_from_ce();
            let random_days = fastrand::i32(from_days..=to_days);
            let date = NaiveDate::from_num_days_from_ce_opt(random_days)
                .expect("valid day count from random range");
            date.format("%d-%m-%Y").to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_generate_birthday_within_range() {
        let range = DateRange {
            from: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(1990, 12, 31).unwrap(),
        };

        let generator = BirthdayGenerator;

        for _ in 0..100 {
            let birthday = generator.generate(&range);
            // Check format is DD-MM-YYYY
            assert_eq!(birthday.len(), 10);
            assert!(birthday.contains("-"));

            // Parse back and verify within range
            let parts: Vec<&str> = birthday.split('-').collect();
            let day: u32 = parts[0].parse().unwrap();
            let month: u32 = parts[1].parse().unwrap();
            let year: i32 = parts[2].parse().unwrap();

            let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            assert!(date >= range.from);
            assert!(date <= range.to);
        }
    }

    #[test]
    fn test_generate_birthday_single_day() {
        let date = NaiveDate::from_ymd_opt(1995, 6, 15).unwrap();
        let range = DateRange {
            from: date,
            to: date,
        };

        let generator = BirthdayGenerator;
        let birthday = generator.generate(&range);

        assert_eq!(birthday, "15-06-1995");
    }

    #[test]
    fn test_birthday_format_dd_mm_yyyy() {
        let range = DateRange {
            from: NaiveDate::from_ymd_opt(2000, 1, 5).unwrap(),
            to: NaiveDate::from_ymd_opt(2000, 1, 5).unwrap(),
        };

        let generator = BirthdayGenerator;
        let birthday = generator.generate(&range);

        // Should have leading zeros: 05-01-2000
        assert_eq!(birthday, "05-01-2000");
    }
}
