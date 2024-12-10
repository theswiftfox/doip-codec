use thiserror::Error;

use crate::doip::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_MESSAGE_ACK_CODE_LEN,
    },
    message::diagnostic_ack::DiagnosticAckCode,
};

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct DiagnosticMessageAck {
    pub source_address: [u8; 2],
    pub target_address: [u8; 2],
    pub ack_code: DiagnosticAckCode,
}

impl DoipPayload for DiagnosticMessageAck {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessageAck
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&self.target_address);
        bytes.extend_from_slice(&[self.ack_code as u8]);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::DiagnosticMessageAckError(
                DiagnosticMessageAckError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageAckError(
                        DiagnosticMessageAckError::InvalidIndexRange,
                    ))
                }
            };

        let target_address_offset = source_address_offset + DOIP_DIAG_COMMON_TARGET_LEN;
        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] =
            match bytes[source_address_offset..target_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageAckError(
                        DiagnosticMessageAckError::InvalidIndexRange,
                    ))
                }
            };

        let _ack_code_offset = target_address_offset + DOIP_DIAG_MESSAGE_ACK_CODE_LEN;
        let ack_code = match &bytes[target_address_offset] {
            0x00 => DiagnosticAckCode::Acknowledged,
            _ => {
                return Err(PayloadError::DiagnosticMessageAckError(
                    DiagnosticMessageAckError::InvalidAckCode,
                ))
            }
        };

        Ok(Self {
            source_address,
            target_address,
            ack_code,
        })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum DiagnosticMessageAckError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
    #[error("invalid acknowledgement code")]
    InvalidAckCode,
}

#[cfg(test)]
mod tests {
    use crate::doip::{
        header::payload::{
            diagnostic_message_ack::{DiagnosticMessageAck, DiagnosticMessageAckError},
            payload::{DoipPayload, PayloadError, PayloadType},
        },
        message::diagnostic_ack::DiagnosticAckCode,
    };

    const DEFAULT_SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const DEFAULT_TARGET_ADDRESS: [u8; 2] = [0x03, 0x04];
    const DEFAULT_ACK_CODE: DiagnosticAckCode = DiagnosticAckCode::Acknowledged;

    #[test]
    fn test_payload_type() {
        let request = DiagnosticMessageAck {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            ack_code: DEFAULT_ACK_CODE,
        };
        assert_eq!(request.payload_type(), PayloadType::DiagnosticMessageAck);
    }

    #[test]
    fn test_to_bytes() {
        let request = DiagnosticMessageAck {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            ack_code: DEFAULT_ACK_CODE,
        };
        assert_eq!(request.to_bytes(), vec![0x01, 0x02, 0x03, 0x04, 0x00]);
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = vec![0x01, 0x02, 0x03];
        let from_bytes = DiagnosticMessageAck::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessageAckError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageAckError(DiagnosticMessageAckError::InvalidLength),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_invalid_ack_code() {
        let request = vec![0x01, 0x02, 0x03, 0x04, 0x01];
        let from_bytes = DiagnosticMessageAck::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessageAckError::InvalidAckCode."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageAckError(DiagnosticMessageAckError::InvalidAckCode),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let request = DiagnosticMessageAck {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            ack_code: DEFAULT_ACK_CODE,
        }
        .to_bytes();
        let from_bytes = DiagnosticMessageAck::from_bytes(&request);

        assert!(
            from_bytes.is_ok(),
            "Expected DiagnosticMessageAck, recieved an Error."
        );
    }
}
