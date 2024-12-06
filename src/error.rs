use std::io;

pub enum ParseError {
    EmptyInput,
    InvalidProtocolVersion,
    FailedProtocolCheck,
    PayloadNotRecognised,
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Underlying I/O Error: {0}")]
    IoError(#[from] io::Error),
}