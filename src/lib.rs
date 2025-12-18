mod cli;
mod credentials;
mod name;
mod output;

use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Context;
use clap::Parser;

pub use cli::Args;
pub use credentials::{CredentialConfig, CredentialEnricher, PersonRecord};
pub use name::{EmbeddedNameSource, Gender, NameConfig, NameGenerator, NameParts, NameSource};
pub use output::{
    create_formatter, create_person_formatter, NameFormatter, OutputFormat, PersonFormatter,
};

/// Main application entry point.
pub fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let name_config = NameConfig {
        gender: args.gender.into(),
        include_middle_name: args.middle,
        allow_duplicates: args.allow_duplicates,
    };

    let generator = NameGenerator::new(EmbeddedNameSource);
    let names = generator
        .generate_many(&name_config, args.count)
        .with_context(|| "run::0")?;

    // Determine if we need credentials
    let needs_credentials = args.email || args.password;

    let output = if needs_credentials {
        let credential_config = CredentialConfig {
            generate_email: args.email,
            generate_password: args.password,
            email_domain: args.domain,
        };

        let enricher = CredentialEnricher::new(&credential_config);
        let records = enricher.enrich_many(names);

        let formatter = create_person_formatter(args.format.into());
        formatter.format(&records).with_context(|| "run::1")?
    } else {
        let formatter = create_formatter(args.format.into());
        formatter.format(&names).with_context(|| "run::1")?
    };

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
