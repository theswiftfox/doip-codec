use thiserror::Error;

use crate::doip::{definitions::DOIP_POWER_MODE_LEN, message::power_mode::PowerMode};

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct PowerInformationResponse {
    pub power_mode: PowerMode,
}

impl DoipPayload for PowerInformationResponse {
    fn payload_type(&self) -> PayloadType {
        PayloadType::PowerInformationResponse
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&[self.power_mode as u8]);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_POWER_MODE_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::PowerInformationResponseError(
                PowerInformationResponseError::InvalidLength,
            ));
        }

        let power_mode_offset = 0;
        let power_mode = match &bytes[power_mode_offset] {
            0x00 => PowerMode::NotReady,
            0x01 => PowerMode::Ready,
            0x02 => PowerMode::NotSupported,
            _ => {
                return Err(PayloadError::PowerInformationResponseError(
                    PowerInformationResponseError::InvalidPowerMode,
                ))
            }
        };

        Ok(Self { power_mode })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum PowerInformationResponseError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
    #[error("powermode not supported")]
    InvalidPowerMode,
}

#[cfg(test)]
mod tests {
    use crate::doip::{
        header::payload::{
            payload::{DoipPayload, PayloadError, PayloadType},
            power_information_response::{PowerInformationResponse, PowerInformationResponseError},
        },
        message::power_mode::PowerMode,
    };

    const DEFAULT_POWER_MODE: PowerMode = PowerMode::NotReady;

    #[test]
    fn test_payload_type() {
        let request = PowerInformationResponse {
            power_mode: DEFAULT_POWER_MODE,
        };
        assert_eq!(
            request.payload_type(),
            PayloadType::PowerInformationResponse
        );
    }

    #[test]
    fn test_to_bytes() {
        let request = PowerInformationResponse {
            power_mode: DEFAULT_POWER_MODE,
        };
        assert_eq!(request.to_bytes(), vec![0x00]);
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = vec![];
        let from_bytes = PowerInformationResponse::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an PowerInformationResponseError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::PowerInformationResponseError(
                PowerInformationResponseError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_invalid_power_mode() {
        let request = vec![0x03];
        let from_bytes = PowerInformationResponse::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an PowerInformationResponseError::InvalidAckCode."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::PowerInformationResponseError(
                PowerInformationResponseError::InvalidPowerMode
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let request = PowerInformationResponse {
            power_mode: DEFAULT_POWER_MODE,
        }
        .to_bytes();
        let from_bytes = PowerInformationResponse::from_bytes(&request);

        assert!(
            from_bytes.is_ok(),
            "Expected PowerInformationResponse, recieved an Error."
        );
    }
}
