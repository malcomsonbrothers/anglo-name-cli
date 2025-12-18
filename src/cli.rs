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

    /// Sex for first names (male, female, or any)
    #[arg(short, long, value_enum, default_value = "any")]
    pub sex: SexArg,

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

    /// Start date for birthday range (DD-MM-YYYY)
    #[arg(long, requires = "birthday_to", conflicts_with_all = ["age", "age_from", "age_to"])]
    pub birthday_from: Option<String>,

    /// End date for birthday range (DD-MM-YYYY)
    #[arg(long, requires = "birthday_from", conflicts_with_all = ["age", "age_from", "age_to"])]
    pub birthday_to: Option<String>,

    /// Single age - everyone is exactly this age (years old today)
    #[arg(long, conflicts_with_all = ["birthday_from", "birthday_to", "age_from", "age_to"])]
    pub age: Option<u32>,

    /// Minimum age for age range
    #[arg(long, requires = "age_to", conflicts_with_all = ["birthday_from", "birthday_to", "age"])]
    pub age_from: Option<u32>,

    /// Maximum age for age range
    #[arg(long, requires = "age_from", conflicts_with_all = ["birthday_from", "birthday_to", "age"])]
    pub age_to: Option<u32>,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum SexArg {
    Male,
    Female,
    #[default]
    Any,
}

impl From<SexArg> for Gender {
    fn from(arg: SexArg) -> Self {
        match arg {
            SexArg::Male => Gender::Male,
            SexArg::Female => Gender::Female,
            SexArg::Any => Gender::Any,
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
