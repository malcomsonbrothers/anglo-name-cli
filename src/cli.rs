use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::name::Gender;
use crate::output::OutputFormat;

#[derive(Parser, Debug)]
#[command(name = "anglo-name")]
#[command(about = "Generate random Anglo names")]
#[command(version)]
pub struct Args {
    /// Number of names to generate
    #[arg(short = 'n', long, default_value = "1")]
    pub count: usize,

    /// Gender for first names (male, female, or any)
    #[arg(short, long, value_enum, default_value = "any")]
    pub gender: GenderArg,

    /// Include middle name
    #[arg(short, long)]
    pub middle: bool,

    /// Output format (plain, json, csv)
    #[arg(short = 'f', long, value_enum, default_value = "plain")]
    pub format: FormatArg,

    /// Output file (stdout if not specified)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Allow duplicate names (by default all names are unique)
    #[arg(long)]
    pub allow_duplicates: bool,

    /// Generate email addresses for each name
    #[arg(short = 'e', long)]
    pub email: bool,

    /// Generate passwords for each name
    #[arg(short = 'p', long)]
    pub password: bool,

    /// Email domain (default: random gmail.com or outlook.com)
    #[arg(short = 'd', long, requires = "email")]
    pub domain: Option<String>,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum GenderArg {
    Male,
    Female,
    #[default]
    Any,
}

impl From<GenderArg> for Gender {
    fn from(arg: GenderArg) -> Self {
        match arg {
            GenderArg::Male => Gender::Male,
            GenderArg::Female => Gender::Female,
            GenderArg::Any => Gender::Any,
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum FormatArg {
    #[default]
    Plain,
    Json,
    Csv,
}

impl From<FormatArg> for OutputFormat {
    fn from(arg: FormatArg) -> Self {
        match arg {
            FormatArg::Plain => OutputFormat::Plain,
            FormatArg::Json => OutputFormat::Json,
            FormatArg::Csv => OutputFormat::Csv,
        }
    }
}
