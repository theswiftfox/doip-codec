use thiserror::Error;

use crate::doip::definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN};

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Clone, Debug)]
pub struct DiagnosticMessage {
    pub source_address: [u8; 2],
    pub target_address: [u8; 2],
    pub message: Vec<u8>,
}

impl DoipPayload for DiagnosticMessage {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessage
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&self.target_address);
        bytes.extend_from_slice(&self.message);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::DiagnosticMessageError(
                DiagnosticMessageError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageError(
                        DiagnosticMessageError::InvalidIndexRange,
                    ))
                }
            };

        let target_address_offset = source_address_offset + DOIP_DIAG_COMMON_TARGET_LEN;
        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] =
            match bytes[source_address_offset..target_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageError(
                        DiagnosticMessageError::InvalidIndexRange,
                    ))
                }
            };

        let message = bytes[target_address_offset..].to_vec();

        Ok(Self {
            source_address,
            target_address,
            message,
        })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum DiagnosticMessageError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}

#[cfg(test)]
mod tests {
    use crate::doip::header::payload::{
        diagnostic_message::{DiagnosticMessage, DiagnosticMessageError},
        payload::{DoipPayload, PayloadError, PayloadType},
    };

    const DEFAULT_SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const DEFAULT_TARGET_ADDRESS: [u8; 2] = [0x03, 0x04];

    #[test]
    fn test_payload_type() {
        let request = DiagnosticMessage {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            message: vec![0x05, 0x06, 0x07, 0x08],
        };
        assert_eq!(request.payload_type(), PayloadType::DiagnosticMessage);
    }

    #[test]
    fn test_to_bytes() {
        let request = DiagnosticMessage {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            message: vec![0x05, 0x06, 0x07, 0x08],
        };
        assert_eq!(
            request.to_bytes(),
            vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
        );
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = vec![0x01, 0x02, 0x03];
        let from_bytes = DiagnosticMessage::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessage::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageError(DiagnosticMessageError::InvalidLength),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let request = DiagnosticMessage {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            message: vec![0x05, 0x06, 0x07, 0x08],
        }
        .to_bytes();
        let from_bytes = DiagnosticMessage::from_bytes(&request);

        assert!(
            from_bytes.is_ok(),
            "Expected DiagnosticMessage, recieved an Error."
        );
    }
}
