use anyhow::{anyhow, Result};
use toml::{Table, Value};
use toml_schema::schema::{Schema as RefSchema, ValidationError};

use std::path::PathBuf;

pub(super) struct SchemaImpl(RefSchema);

impl SchemaImpl {
    /// use the contents of a reader as the contents of a tosd to
    /// create a schema object
    pub(super) fn from_reader(
        reader: &mut dyn std::io::Read,
    ) -> Result<super::Schema> {
        let contents = {
            let mut c = String::new();
            reader.read_to_string(&mut c)
                .map_err(Into::<anyhow::Error>::into)?;
            c
        };

        let table = toml::from_str::<Table>(&contents)
            .map_err(Into::<anyhow::Error>::into)?;

        Self::from_table(table)
    }

    /// convert an existing toml table into a schema object
    pub(super) fn from_table(table: Table) -> Result<super::Schema> {
        if let Some(Value::Table(toml_schema)) = table.get("toml-schema")
        && let Some(Value::String(version)) = toml_schema.get("version") {
            let meta = super::SchemaMeta {
                version: semver::Version::parse(version).map_err(Into::<anyhow::Error>::into)?,
                meta: toml_schema.get("meta").and_then(Value::as_table).cloned(),
            };
            let schema = RefSchema::from_table(PathBuf::from("<unspecified>"), table)
                .map_err(|s| anyhow!(s))?;

            Ok(super::Schema {
                origin: None,
                meta,
                inner: Self(schema),
            })
        } else {
            Err(anyhow!("from_table(): No `[toml-schema]` table was found in the toml"))
        }
    }

    /// validate an existing toml table either using a provided schema object,
    /// or with a tosd file located using the table's `toml-schema.location`
    /// attribute
    pub(super) fn validate_table(
        table: &Table,
        schema: Option<super::Schema>
    ) -> Result<()> {
        match schema {
            None => {
                if let Some(Value::Table(toml_schema)) = table.get("toml-schema")
                && let Some(Value::String(location)) = toml_schema.get("location")
                {
                    let schema = super::Schema::from_path(&PathBuf::from(location))?;
                    Self::validate_table(table, Some(schema))
                } else {
                    Err(anyhow!("validate_table(): No schema was provided, and no `[toml-schema].location` value was found in the table"))
                }
            },
            Some(schema) => {
                let result = schema.inner.0.validate(table);
                if result.valid() {
                    Ok(())
                } else {
                    let mut err_msg = String::new();

                    result.errors.into_iter()
                        .for_each(|ValidationError { path, message }| {
                            err_msg.push('[');
                            err_msg.push_str(path.as_str());
                            err_msg.push_str("]: ");
                            err_msg.push_str(&message);
                            err_msg.push('\n');
                        });

                    Err(anyhow!(err_msg))
                }
            }
        }
    }
}

