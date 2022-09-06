use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction};

#[derive(Debug)]
pub struct Cli {
    /// Enable more verbose logging
    pub verbose: bool,

    /// Path to cfg.toml
    pub config: Option<PathBuf>,
}

impl Cli {
    pub fn arguments() -> Self {
        let matches = command!()
            .arg(
                arg!(
                    -c --config <FILE> "Sets a custom config file"
                )
                .required(false)
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -v --verbose "Enable more verbose logging"
                )
                .action(ArgAction::SetTrue),
            )
            .get_matches();

        Self {
            verbose: matches
                .get_one::<bool>("verbose")
                .copied()
                .unwrap_or_default(),
            config: matches.get_one::<PathBuf>("config").cloned(),
        }
    }
}
