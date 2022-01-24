use serde_json::Value;
use thiserror::Error;

/// Error enum for the stac crate.
#[derive(Error, Debug)]
pub enum Error {
    /// std::io::Error.
    #[error("std::io error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid type field (not a string).
    #[error("Invalid \"type\" field: {0}")]
    InvalidTypeField(Value),

    /// Invalid type value.
    #[error("Invalid \"type\" value: {0}")]
    InvalidTypeValue(String),

    /// No "type" field in the JSON object, unable to parse as STAC object.
    #[error("No \"type\" field in the JSON object, unable to parse as STAC object")]
    MissingType,

    /// A serde_json error.
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    /// A url parse error.
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
}