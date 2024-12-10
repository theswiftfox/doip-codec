use thiserror::Error;

use crate::doip::definitions::DOIP_DIAG_COMMON_SOURCE_LEN;

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct AliveCheckResponse {
    pub source_address: [u8; 2],
}

impl DoipPayload for AliveCheckResponse {
    fn payload_type(&self) -> PayloadType {
        PayloadType::AliveCheckResponse
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.source_address.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::AliveCheckResponseParseError(
                AliveCheckResponseParseError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(array) => array,
                Err(_) => {
                    return Err(PayloadError::AliveCheckResponseParseError(
                        AliveCheckResponseParseError::InvalidIndexRange,
                    ))
                }
            };

        Ok(Self { source_address })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AliveCheckResponseParseError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}

#[cfg(test)]
mod tests {
    use crate::doip::header::payload::{
        alive_check_response::{AliveCheckResponse, AliveCheckResponseParseError},
        payload::{DoipPayload, PayloadError, PayloadType},
    };

    const SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const INVALID_LENGTH: [u8; 1] = [0x03];

    #[test]
    fn test_payload_type() {
        let request = AliveCheckResponse {
            source_address: SOURCE_ADDRESS,
        };
        assert_eq!(request.payload_type(), PayloadType::AliveCheckResponse);
    }

    #[test]
    fn test_to_bytes() {
        let request = AliveCheckResponse {
            source_address: SOURCE_ADDRESS,
        };
        assert_eq!(request.to_bytes(), SOURCE_ADDRESS.to_vec());
    }

    #[test]
    fn test_from_bytes_invalid_length() {
        let bytes = INVALID_LENGTH;
        let request = AliveCheckResponse::from_bytes(&bytes);

        assert!(
            request.is_err(),
            "Expected to receive an AliveCheckResponseParse::InvalidLength."
        );

        let error = request.unwrap_err();

        assert_eq!(
            error,
            PayloadError::AliveCheckResponseParseError(AliveCheckResponseParseError::InvalidLength),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let bytes = AliveCheckResponse {
            source_address: SOURCE_ADDRESS,
        }
        .to_bytes();

        let request = AliveCheckResponse::from_bytes(&bytes);

        assert!(
            request.is_ok(),
            "Expected AliveCheckResponse, recieved an Error."
        );
    }
}
