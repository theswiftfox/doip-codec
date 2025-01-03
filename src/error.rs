use std::io;

use doip_definitions::error::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Underlying I/O Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Underlying Parse Error: {0}")]
    ParseError(#[from] ParseError),
}

#[derive(thiserror::Error, Debug)]
pub enum EncodeError {
    #[error("Underlying I/O Error: {0}")]
    IoError(#[from] io::Error),
}
