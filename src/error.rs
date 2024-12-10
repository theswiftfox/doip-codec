use std::io;

use thiserror::Error;

use crate::doip::header::payload::payload::PayloadError;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("empty input")]
    EmptyInput,
    #[error("invalid protocol version")]
    InvalidProtocolVersion,
    #[error("failed protocol check")]
    FailedProtocolCheck,
    #[error("payload parse error")]
    PayloadParseError(#[from] PayloadError),
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Underlying I/O Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Underlying Parse Error: {0}")]
    ParseError(#[from] ParseError),
}
