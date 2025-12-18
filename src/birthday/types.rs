use chrono::NaiveDate;

/// Validated date range for birthday generation.
#[derive(Debug, Clone)]
pub struct DateRange {
    pub from: NaiveDate,
    pub to: NaiveDate,
}
