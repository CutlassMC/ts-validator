use anyhow::Result;
use clap_complete::aot::Generator;
use ovation::{CommandContext, CommandDelegate, CommandSet};
use ts_validator_lib::Schema;

use crate::args::{Args, ValidatorAction};

impl CommandSet<Args> for ValidatorAction {
    type ReturnType = ();
    type ErrorType = anyhow::Error;

    fn dispatch<'a>(&self) -> &'a dyn CommandDelegate<Args> {
        match self {
            ValidatorAction::ValidateToml { .. } => &validate_toml,
            ValidatorAction::ValidateTosd { .. } => &validate_tosd,
            ValidatorAction::Completions { .. } => &completions,
        }
    }
}

impl CommandContext for Args {
    type Commands = ValidatorAction;

    fn commands(&self) -> &<Self as CommandContext>::Commands {
        &self.subcommands
    }
}

fn validate_toml(_: &Args, set: &ValidatorAction) -> Result<()> {
    if let ValidatorAction::ValidateToml { schema_path, toml_path } = set {
        let table: toml::Table = toml::from_str(&toml_path.read_to_string()?)?;

        let schema = schema_path.as_ref()
            .map(std::convert::AsRef::as_ref)
            .map(Schema::from_path)
            .transpose()?;

        Schema::validate_table(&table, schema)
    } else {
        // Safety: This is provably safe because it's a private function
        // that is only called as a return from the corresponding variant of
        // ValidatorAction's CommandSet::dispatch implementation.
        unsafe {
            std::hint::unreachable_unchecked()
        }
    }
}

fn validate_tosd(_: &Args, set: &ValidatorAction) -> Result<()> {
    if let ValidatorAction::ValidateTosd { tosd_path } = set {
        let tosd_contents = tosd_path.read_to_string()?;

        let table: toml::Table = toml::from_str(&tosd_contents)?;

        Schema::validate_table(&table, Some(canon_schema()))?;

        Schema::from_table(table).map(|_| ())
    } else {
        // Safety: This is provably safe because it's a private function
        // that is only called as a return from the corresponding variant of
        // ValidatorAction's CommandSet::dispatch implementation.
        unsafe {
            std::hint::unreachable_unchecked()
        }
    }
}

fn canon_schema() -> Schema {
    const CANON_SCHEMA_CONTENTS: &str = include_str!("../toml-schema.tosd");

    Schema::from_table(toml::from_str(CANON_SCHEMA_CONTENTS).unwrap()).unwrap()
}

fn completions(_: &Args, set: &ValidatorAction) -> Result<()> {
    if let ValidatorAction::Completions { file, shell } = set {
        let path = format!(
            "{}/{}",
            std::env::var("COMPLETIONS_DIR").unwrap_or_else(|_| env!("OUT_DIR").to_string()),
            Generator::file_name(shell, clap::crate_name!())
        );

        if *file {
            println!("{}", path);
            Ok(())
        } else {
            let s = std::fs::read_to_string(path).map_err(Into::<anyhow::Error>::into)?;

            print!("{s}");
            Ok(())
        }
    } else {
        // Safety: This is provably safe because it's a private function
        // that is only called as a return from the corresponding variant of
        // ValidatorAction's CommandSet::dispatch implementation.
        unsafe {
            std::hint::unreachable_unchecked()
        }
    }
}

impl Args {
    pub fn execute() -> ovation::err::OvationResult<Args> {
        CommandContext::execute()
    }
}

