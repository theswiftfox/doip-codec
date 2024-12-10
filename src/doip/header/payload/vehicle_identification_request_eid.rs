use thiserror::Error;

use crate::doip::definitions::DOIP_COMMON_EID_LEN;

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestEid {
    pub eid: [u8; DOIP_COMMON_EID_LEN],
}

impl DoipPayload for VehicleIdentificationRequestEid {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequestEid
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.eid.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_EID_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::VehicleIdentificationRequestEidError(
                VehicleIdentificationRequestEidError::InvalidLength,
            ));
        }

        let eid_offset = DOIP_COMMON_EID_LEN;
        let eid: [u8; DOIP_COMMON_EID_LEN] = match bytes[0..eid_offset].try_into() {
            Ok(arr) => arr,
            Err(_) => {
                return Err(PayloadError::VehicleIdentificationRequestEidError(
                    VehicleIdentificationRequestEidError::InvalidIndexRange,
                ))
            }
        };

        Ok(Self { eid })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum VehicleIdentificationRequestEidError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}
