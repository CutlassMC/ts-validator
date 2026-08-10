use clap::{Parser, Subcommand};

#[derive(Subcommand)]
#[clap(name = "action")]
pub enum ValidatorAction {
    /// Extract a schema prototype from a toml file, assuming its values' types
    /// as the schema's value types.
    Extract {
        /// The toml file to generate a schema prototype for.
        ///
        /// Use `-` to read from stdin.
        toml_path: patharg::InputArg,

        /// Where to write the schema file to.
        ///
        /// Use `-` for stdout.
        #[clap(name = "out", default_value = "-")]
        out_path: patharg::OutputArg,
    },

    /// Validate a toml file against a provided schema. Defaults to using
    /// `toml-schema.location` as the schema to read.
    ValidateToml {
        /// The schema to validate against. If not provided,
        /// `toml-schema.location` must be present in the target toml.
        #[arg(name = "schema", short, long)]
        schema_path: Option<patharg::InputArg>,

        /// The toml file to validate. Defaults to reading from stdin.
        #[clap(name = "toml", default_value = "-")]
        toml_path: patharg::InputArg,
    },

    /// Check if a tosd file is valid.
    ValidateTosd {
        /// The tosd file to validate. Defaults to reading from stdin.
        #[clap(name = "tosd", default_value = "-")]
        tosd_path: patharg::InputArg,
    },

    /// Return shell completions
    Completions {
        /// Return a filepath to the completion instead of writing the
        /// contents to stdout.
        #[arg(short, long)]
        file: bool,

        /// Which shell to get the completions for.
        #[arg(value_parser)]
        shell: clap_complete::aot::Shell,
    },
}

/// Validate a toml file using a provided TOml Schema Definition (TOSD).
/// This will validate a toml source against a given schema, or extract
/// a schema prototype from an existing toml file.
///
/// Reference: https://toml-schema.org/
#[derive(Parser)]
#[clap(version, about)]
pub struct Args {
    /// Don't print anything to stdout. Will still print usage errors.
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    #[command(subcommand)]
    pub subcommands: ValidatorAction,
}
