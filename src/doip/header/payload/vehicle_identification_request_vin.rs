use crate::doip::definitions::DOIP_COMMON_VIN_LEN;

use super::payload::{DoipPayload, PayloadType};

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

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_VIN_LEN;

        if bytes.len() < min_length {
            return None;
        }

        let vin_offset = DOIP_COMMON_VIN_LEN;
        let vin: [u8; DOIP_COMMON_VIN_LEN] = bytes[0..vin_offset].try_into().ok()?;

        Some(Self { vin })
    }
}
