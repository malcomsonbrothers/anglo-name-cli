mod csv;
mod format;
mod json;
mod plain;

pub use format::{NameFormatter, OutputFormat};

use csv::CsvFormatter;
use json::JsonFormatter;
use plain::PlainFormatter;

/// Create a formatter for the given output format.
pub fn create_formatter(format: OutputFormat) -> Box<dyn NameFormatter> {
    match format {
        OutputFormat::Plain => Box::new(PlainFormatter),
        OutputFormat::Json => Box::new(JsonFormatter),
        OutputFormat::Csv => Box::new(CsvFormatter),
    }
}
