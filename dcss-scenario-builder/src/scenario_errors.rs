use dcss_api::Error as APIError;
use thiserror::Error;

/// Main errors types that can be raised while using the scenario builder.
#[derive(Error)]
pub enum Error {
    #[error(transparent)]
    APIError(#[from] APIError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error("DCSS Parsing Error: {0}")]
    YamlParsingError(#[from] YamlParsingError),
    #[error("DCSS Lua error: {0}")]
    LuaError(String),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;
        Ok(())
    }
}

/// Errors that can be raised while parsing the yaml scenarios.
#[derive(Error, Debug)]
pub enum YamlParsingError {
    #[error("Branch `{0}` does not exist.")]
    UnknownBranch(String),
    #[error("Missing `@` on `D:1` in the yaml.")]
    MissingChar,
    #[error("Maximum width of map is 79 columns.")]
    MapTooWide,
    #[error("Maximum height of map is 69 rows.")]
    MapTooLong,
}
