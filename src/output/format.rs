use crate::credentials::PersonRecord;
use crate::name::NameParts;

/// Supported output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Plain,
    Json,
    Csv,
}

/// Trait for formatting name output.
pub trait NameFormatter: Send + Sync {
    /// Format a collection of names for output.
    fn format(&self, names: &[NameParts]) -> anyhow::Result<String>;
}

/// Trait for formatting person record output (names with optional credentials).
pub trait PersonFormatter: Send + Sync {
    /// Format a collection of person records for output.
    fn format(&self, records: &[PersonRecord]) -> anyhow::Result<String>;
}
