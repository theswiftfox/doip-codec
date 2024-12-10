use thiserror::Error;

use super::payload::{DoipPayload, PayloadError, PayloadType};
use crate::doip::{definitions::DOIP_GENERIC_NACK_LEN, message::nack_codes::NackCodes};

#[derive(Copy, Clone, Debug)]
pub struct GenericNack {
    pub nack_code: NackCodes,
}

impl DoipPayload for GenericNack {
    fn payload_type(&self) -> PayloadType {
        PayloadType::GenericNack
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let nc = [(self.nack_code as u8)];

        bytes.extend_from_slice(&nc);
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_GENERIC_NACK_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::GenericNackError(
                GenericNackError::InvalidLength,
            ));
        }

        let nack_code_offset = 0;
        let nack_code = match &bytes[nack_code_offset] {
            0x00 => NackCodes::IncorrectPatternFormat,
            0x01 => NackCodes::UnknownPayloadType,
            0x02 => NackCodes::MessageTooLarge,
            0x03 => NackCodes::OutOfMemory,
            0x04 => NackCodes::InvalidPayloadLength,
            _ => {
                return Err(PayloadError::GenericNackError(
                    GenericNackError::InvalidNackCode,
                ))
            }
        };

        Ok(Self { nack_code })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GenericNackError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
    #[error("invalid nack code")]
    InvalidNackCode,
}

#[cfg(test)]
mod tests {
    use crate::doip::{
        header::payload::{
            generic_nack::{GenericNack, GenericNackError},
            payload::{DoipPayload, PayloadError, PayloadType},
        },
        message::nack_codes::NackCodes,
    };

    const DEFAULT_NACK_CODE: NackCodes = NackCodes::IncorrectPatternFormat;

    #[test]
    fn test_payload_type() {
        let request = GenericNack {
            nack_code: DEFAULT_NACK_CODE,
        };
        assert_eq!(request.payload_type(), PayloadType::GenericNack);
    }

    #[test]
    fn test_to_bytes() {
        let request = GenericNack {
            nack_code: DEFAULT_NACK_CODE,
        };
        assert_eq!(request.to_bytes(), vec![0x00]);
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = vec![];
        let from_bytes = GenericNack::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an GenericNackError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::GenericNackError(GenericNackError::InvalidLength),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_invalid_nack_code() {
        let request = vec![0x05];
        let from_bytes = GenericNack::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an GenericNackError::InvalidNackCode."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::GenericNackError(GenericNackError::InvalidNackCode),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let request = GenericNack {
            nack_code: DEFAULT_NACK_CODE,
        }
        .to_bytes();
        let from_bytes = GenericNack::from_bytes(&request);

        assert!(
            from_bytes.is_ok(),
            "Expected GenericNack, recieved an Error."
        );
    }
}
