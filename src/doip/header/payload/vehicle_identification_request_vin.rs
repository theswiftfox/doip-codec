use thiserror::Error;

use crate::doip::definitions::DOIP_COMMON_VIN_LEN;

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestVin {
    pub vin: [u8; DOIP_COMMON_VIN_LEN],
}

impl DoipPayload for VehicleIdentificationRequestVin {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequestVin
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.vin.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_VIN_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::VehicleIdentificationRequestVinError(
                VehicleIdentificationRequestVinError::InvalidLength,
            ));
        }

        let vin_offset = DOIP_COMMON_VIN_LEN;
        let vin: [u8; DOIP_COMMON_VIN_LEN] = match bytes[0..vin_offset].try_into() {
            Ok(arr) => arr,
            Err(_) => {
                return Err(PayloadError::VehicleIdentificationRequestVinError(
                    VehicleIdentificationRequestVinError::InvalidIndexRange,
                ))
            }
        };

        Ok(Self { vin })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum VehicleIdentificationRequestVinError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}

#[cfg(test)]
mod tests {
    use crate::doip::{
        definitions::DOIP_COMMON_VIN_LEN,
        header::payload::{
            payload::{DoipPayload, PayloadError, PayloadType}, vehicle_identification_request_vin::{VehicleIdentificationRequestVin, VehicleIdentificationRequestVinError},
        },
    };

    const DEFAULT_VIN: [u8; DOIP_COMMON_VIN_LEN] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11,
    ];

    #[test]
    fn test_payload_type() {
        let request = VehicleIdentificationRequestVin { vin: DEFAULT_VIN };
        assert_eq!(
            request.payload_type(),
            PayloadType::VehicleIdentificationRequestVin
        );
    }

    #[test]
    fn test_to_bytes() {
        let request = VehicleIdentificationRequestVin { vin: DEFAULT_VIN };
        assert_eq!(request.to_bytes(), DEFAULT_VIN.to_vec());
    }

    #[test]
    fn test_from_bytes_invalid_length() {
        let bytes = [0x00, 0x01, 0x02, 0x03, 0x04];
        let request = VehicleIdentificationRequestVin::from_bytes(&bytes);

        assert!(
            request.is_err(),
            "Expected to receive an VehicleIdentificationRequestVinParse::InvalidLength."
        );

        let error = request.unwrap_err();

        assert_eq!(
            error,
            PayloadError::VehicleIdentificationRequestVinError(
                VehicleIdentificationRequestVinError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let bytes = VehicleIdentificationRequestVin { vin: DEFAULT_VIN }.to_bytes();
        let request = VehicleIdentificationRequestVin::from_bytes(&bytes);

        assert!(
            request.is_ok(),
            "Expected VehicleIdentificationRequestVin, recieved an Error."
        );
    }
}
