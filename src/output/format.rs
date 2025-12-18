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
