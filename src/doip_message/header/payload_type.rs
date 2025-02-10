use doip_definitions::header::PayloadType;
use tokio::io;

use crate::DecodeError;

pub const MAX: usize = 8 * 1024 * 1024;

pub struct PayloadTypeCodec;

impl tokio_util::codec::Decoder for PayloadTypeCodec {
    type Item = PayloadType;
    type Error = PayloadTypeError;

    fn decode(
        &mut self,
        src: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        };

        if src.len() > MAX {
            return Err(DecodeError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", src.len()),
            )));
        }

        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PayloadTypeError {
    /// IO error from Stream
    #[error("Underlying I/O Error: {0}")]
    IoError(#[from] io::Error),

    /// Urecognised payload type in buffer.
    #[error("invalid payload type")]
    InvalidPayloadType,

    /// Invalid protocol version.
    #[error("invalid protocol version")]
    InvalidProtocolVersion,

    /// Failed protocol check.
    #[error("failed protocol check")]
    FailedProtocolCheck,
}
