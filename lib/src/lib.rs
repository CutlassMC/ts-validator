#[cfg(feature = "const-impl")]
mod const_backend;

#[cfg(feature = "ref-impl")]
mod ref_backend;

#[cfg(not(feature = "backend"))]
compile_error!(
    "No backend is implemented, which is the result of an invalid configuration of the crate"
);

#[cfg(feature = "const-impl")]
use const_backend as backend;

#[cfg(feature = "ref-impl")]
use ref_backend as backend;

pub mod util;

use anyhow::Result;

use std::path::{Path, PathBuf};

#[repr(C)]
pub struct SchemaMeta {
    pub version: semver::Version,
    pub meta: Option<toml::Table>,
}

#[repr(C)]
pub struct Schema {
    pub origin: Option<String>,
    pub meta: SchemaMeta,
    inner: backend::SchemaImpl,
}

pub struct SchemaUsageMeta {
    pub location: PathBuf,
}

pub struct ValidatedTable {
    pub toml_schema: Option<SchemaUsageMeta>,
    pub table: toml::Table,
}

impl Schema {
    /// use the contents of a reader as the contents of a tosd to
    /// create a schema object
    pub fn from_reader(reader: &mut dyn std::io::Read) -> Result<Self> {
        backend::SchemaImpl::from_reader(reader)
    }

    /// read a .tosd file to create a schema object
    #[inline]
    pub fn from_path(path: &Path) -> Result<Self> {
        let mut file = std::fs::File::open(path)
            .map_err(Into::<anyhow::Error>::into)?;

        Self::from_reader(&mut file)
    }

    /// convert an existing toml table into a schema object
    pub fn from_table(table: toml::Table) -> Result<Self> {
        backend::SchemaImpl::from_table(table)
    }

    /// read a toml file at the provided path and validate it either
    /// with a provided schema object or with a tosd file located using
    /// the toml file's `toml-schema.location` attribute
    pub fn validate_table_at_path(
        path: &Path,
        schema: Option<Self>
    ) -> Result<()> {
        Self::validate_table(
            &toml::from_str(&std::fs::read_to_string(path)?)?,
            schema
        )
    }

    pub fn validate_table_from_reader(
        reader: &mut dyn std::io::Read,
        schema: Option<Self>,
    ) -> Result<()> {
        let mut table_contents = String::new();
        reader.read_to_string(&mut table_contents)
            .map_err(Into::<anyhow::Error>::into)?;

        let table = toml::from_str::<toml::Table>(&table_contents)
            .map_err(Into::<anyhow::Error>::into)?;

        Self::validate_table(&table, schema)
    }

    /// validate an existing toml table either using a provided schema object,
    /// or with a tosd file located using the table's `toml-schema.location`
    /// attribute
    pub fn validate_table(
        table: &toml::Table,
        schema: Option<Self>
    ) -> Result<()> {
        backend::SchemaImpl::validate_table(table, schema)
    }
}

