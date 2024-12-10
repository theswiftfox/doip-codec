use thiserror::Error;

use crate::doip::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_MESSAGE_NACK_CODE_LEN,
    },
    message::diagnostic_nack::DiagnosticNackCode,
};

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct DiagnosticMessageNack {
    pub source_address: [u8; 2],
    pub target_address: [u8; 2],
    pub nack_code: DiagnosticNackCode,
}

impl DoipPayload for DiagnosticMessageNack {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessageNack
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&self.target_address);
        bytes.extend_from_slice(&[self.nack_code as u8]);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_DIAG_COMMON_TARGET_LEN
            + DOIP_DIAG_MESSAGE_NACK_CODE_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::DiagnosticMessageNackParseError(
                DiagnosticMessageNackParseError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(array) => array,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageNackParseError(
                        DiagnosticMessageNackParseError::InvalidIndexRange,
                    ))
                }
            };

        let target_address_offset = source_address_offset + DOIP_DIAG_COMMON_TARGET_LEN;
        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] =
            match bytes[source_address_offset..target_address_offset].try_into() {
                Ok(array) => array,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageNackParseError(
                        DiagnosticMessageNackParseError::InvalidIndexRange,
                    ))
                }
            };

        let nack_code_offset = target_address_offset;
        let nack_code = match &bytes[nack_code_offset] {
            0x00 => DiagnosticNackCode::ReservedByIso13400_00,
            0x01 => DiagnosticNackCode::ReservedByIso13400_01,
            0x02 => DiagnosticNackCode::InvalidSourceAddress,
            0x03 => DiagnosticNackCode::UnknownTargetAddress,
            0x04 => DiagnosticNackCode::DiagnosticMessageTooLarge,
            0x05 => DiagnosticNackCode::OutOfMemory,
            0x06 => DiagnosticNackCode::TargetUnreachable,
            0x07 => DiagnosticNackCode::UnknownNetwork,
            0x08 => DiagnosticNackCode::TransportProtocolError,
            _ => {
                return Err(PayloadError::DiagnosticMessageNackParseError(
                    DiagnosticMessageNackParseError::InvalidNackCode,
                ))
            }
        };

        Ok(Self {
            source_address,
            target_address,
            nack_code,
        })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum DiagnosticMessageNackParseError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
    #[error("invalid negative acknowledgement code")]
    InvalidNackCode,
}

#[cfg(test)]
mod tests {
    use crate::doip::{
        header::payload::{
            diagnostic_message_nack::{DiagnosticMessageNack, DiagnosticMessageNackParseError},
            payload::{DoipPayload, PayloadError, PayloadType},
        },
        message::diagnostic_nack::DiagnosticNackCode,
    };

    const DEFAULT_SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const DEFAULT_TARGET_ADDRESS: [u8; 2] = [0x03, 0x04];
    const DEFAULT_NACK_CODE: DiagnosticNackCode = DiagnosticNackCode::ReservedByIso13400_00;

    #[test]
    fn test_payload_type() {
        let request = DiagnosticMessageNack {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            nack_code: DEFAULT_NACK_CODE,
        };
        assert_eq!(request.payload_type(), PayloadType::DiagnosticMessageNack);
    }

    #[test]
    fn test_to_bytes() {
        let request = DiagnosticMessageNack {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            nack_code: DEFAULT_NACK_CODE,
        };
        assert_eq!(request.to_bytes(), vec![0x01, 0x02, 0x03, 0x04, 0x00]);
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = vec![0x01, 0x02, 0x03];
        let from_bytes = DiagnosticMessageNack::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessageNackParseError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageNackParseError(
                DiagnosticMessageNackParseError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }
    #[test]
    fn test_from_bytes_invalid_nack_code() {
        let request = vec![0x01, 0x02, 0x03, 0x04, 0x09];
        let from_bytes = DiagnosticMessageNack::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessageNackParseError::InvalidAckCode."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageNackParseError(
                DiagnosticMessageNackParseError::InvalidNackCode
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let request = DiagnosticMessageNack {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            nack_code: DEFAULT_NACK_CODE,
        }
        .to_bytes();
        let from_bytes = DiagnosticMessageNack::from_bytes(&request);

        assert!(
            from_bytes.is_ok(),
            "Expected DiagnosticMessageNack, recieved an Error."
        );
    }
}
