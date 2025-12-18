mod cli;
mod name;
mod output;

use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Context;
use clap::Parser;

pub use cli::Args;
pub use name::{EmbeddedNameSource, Gender, NameConfig, NameGenerator, NameParts, NameSource};
pub use output::{create_formatter, NameFormatter, OutputFormat};

/// Main application entry point.
pub fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = NameConfig {
        gender: args.gender.into(),
        include_middle_name: args.middle,
        allow_duplicates: args.allow_duplicates,
    };

    let generator = NameGenerator::new(EmbeddedNameSource);
    let names = generator
        .generate_many(&config, args.count)
        .with_context(|| "run::0")?;

    let formatter = create_formatter(args.format.into());
    let output = formatter.format(&names).with_context(|| "run::1")?;

    match args.output {
        Some(path) => {
            let file = File::create(&path)
                .with_context(|| format!("run::2 - failed to create file {:?}", path))?;
            let mut writer = BufWriter::new(file);
            writeln!(writer, "{}", output)
                .with_context(|| format!("run::3 - failed to write to file {:?}", path))?;
        }
        None => {
            println!("{}", output);
        }
    }

    Ok(())
}
