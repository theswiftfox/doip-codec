use std::io;

use doip_definitions::error::ParseError;

/// A wrapper to encapsulate Parser and IO errors which can occur
#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    /// IO error from Stream
    #[error("Underlying I/O Error: {0}")]
    IoError(#[from] io::Error),

    /// Parsing error from Stream
    #[error("Underlying Parse Error: {0}")]
    ParseError(#[from] ParseError),
}

/// A wrapper to encapsulate IO errors which can occur
#[derive(thiserror::Error, Debug)]
pub enum EncodeError {
    /// IO error from Sink
    #[error("Underlying I/O Error: {0}")]
    IoError(#[from] io::Error),
}
