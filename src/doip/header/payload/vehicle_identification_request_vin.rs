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
