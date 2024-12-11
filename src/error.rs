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
    #[error("index failure")]
    IndexFailure,
    #[error("incomplete payload")]
    IncompletePayload,
    #[error("payload parse error")]
    PayloadParse(#[from] PayloadError),
}

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
