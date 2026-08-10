use anyhow::{anyhow, Result};
use clap_complete::aot::Generator;
use ovation::{CommandContext, CommandDelegate, CommandSet};
use toml_schema::schema::{Schema, ValidationError};

use std::path::PathBuf;
use std::sync::LazyLock;

use crate::util::OrStdin;

pub use crate::args::*;

const CANONICAL_SOURCE: &str = include_str!("../../toml-schema.tosd");

static CANONICAL: LazyLock<Schema> = LazyLock::new(|| {
    let table: toml::Table =
        toml::from_str(CANONICAL_SOURCE).expect("Failed to parse toml-schema.tosd");

    Schema::from_table(crate::util::STDIN_PATHBUF.clone(), table)
        .expect("Failed to construct Schema from builtin toml-schema.tosd")
});

#[derive(serde::Deserialize)]
struct SchemaLocation {
    location: PathBuf,
}

#[derive(serde::Deserialize)]
struct ValidatedTable {
    toml_schema: SchemaLocation,
}

impl Args {
    pub fn execute() -> ovation::err::OvationResult<Self> {
        <Self as CommandContext>::execute()
    }
}

impl CommandSet<Args> for ValidatorAction {
    type ReturnType = ();
    type ErrorType = anyhow::Error;

    fn dispatch<'a>(&self) -> &'a dyn CommandDelegate<Args> {
        match self {
            ValidatorAction::Extract { .. } => &extract,
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

fn extract(_: &Args, set: &ValidatorAction) -> Result<()> {
    if let ValidatorAction::Extract {
        out_path,
        toml_path,
    } = set
    {
        let toml_contents = toml_path
            .read_to_string()
            .map_err(Into::<anyhow::Error>::into)?;
        let toml_table =
            toml::from_str::<toml::Table>(&toml_contents).map_err(Into::<anyhow::Error>::into)?;

        out_path
            .write(toml_schema::extract::generate_schema(&toml_table))
            .map_err(Into::<anyhow::Error>::into)
    } else {
        // Safety: This is provably safe, as this function is only ever
        // called as a mapping from the Extract variant. See the
        // implementation of [`Self::dispatch`].
        unsafe { std::hint::unreachable_unchecked() }
    }
}

fn validate_toml(_: &Args, set: &ValidatorAction) -> Result<(), anyhow::Error> {
    if let ValidatorAction::ValidateToml {
        schema_path,
        toml_path,
    } = set
    {
        let toml_contents = toml_path
            .read_to_string()
            .map_err(Into::<anyhow::Error>::into)?;
        let toml_table =
            toml::from_str::<toml::Table>(&toml_contents).map_err(Into::<anyhow::Error>::into)?;

        let (schema_origin, schema_contents): (PathBuf, String) = match schema_path {
            Option::Some(path) => (
                path.path_ref()
                    .cloned()
                    .unwrap_or_else(|| PathBuf::from("<stdin>")),
                path.read_to_string()?,
            ),
            Option::None => {
                let vtable: ValidatedTable = toml::from_str::<ValidatedTable>(&toml_contents)
                    .map_err(|_| anyhow!("No `--schema` value provided and toml doesn't have a `toml-schema.location` value"))?;

                let location = &vtable.toml_schema.location;
                (PathBuf::from(location), std::fs::read_to_string(location)?)
            }
        };
        let schema_table = toml::from_str(&schema_contents).map_err(Into::<anyhow::Error>::into)?;

        let schema = Schema::from_table(schema_origin, schema_table).map_err(|e| anyhow!("{e}"))?;

        _validate_toml(&toml_table, &schema)
    } else {
        // # Safety
        //
        // This is provably safe, as this function is only ever called as a
        // mapping from the Extract variant. See the implementation of
        // [`Self::dispatch`].
        unsafe { std::hint::unreachable_unchecked() }
    }
}

fn validate_tosd(_: &Args, set: &ValidatorAction) -> Result<()> {
    if let ValidatorAction::ValidateTosd { tosd_path } = set {
        let tosd_contents = tosd_path
            .read_to_string()
            .map_err(Into::<anyhow::Error>::into)?;
        let tosd_table =
            toml::from_str::<toml::Table>(&tosd_contents).map_err(Into::<anyhow::Error>::into)?;

        _validate_toml(&tosd_table, &CANONICAL)?;

        Schema::from_table(tosd_path.path_ref().or_stdin().clone(), tosd_table)
            .map(|_| ())
            .map_err(|s| anyhow!("{s}"))
    } else {
        // # Safety
        //
        // This is provably safe, as this function is only ever called as a
        // mapping from the Extract variant. See the implementation of
        // [`Self::dispatch`].
        unsafe { std::hint::unreachable_unchecked() }
    }
}

fn _validate_toml(toml_table: &toml::Table, schema: &Schema) -> Result<()> {
    let result = schema.validate(toml_table);
    if result.valid() {
        Ok(())
    } else {
        let err_msg: String = result
            .errors
            .into_iter()
            .map(|ValidationError { path, message }| format!("[{path}]: {message}"))
            .collect::<Vec<_>>()
            .join("\n");

        Err(anyhow!(err_msg))
    }
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
        // # Safety
        //
        // This is provably safe, as this function is only ever called as a
        // mapping from the Extract variant. See the implementation of
        // [`Self::dispatch`].
        unsafe { std::hint::unreachable_unchecked() }
    }
}
