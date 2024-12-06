use crate::doip::definitions::DOIP_COMMON_EID_LEN;

use super::payload::{DoipPayload, PayloadType};

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

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_EID_LEN;

        if bytes.len() < min_length {
            return None;
        }

        let eid_offset = DOIP_COMMON_EID_LEN;
        let eid: [u8; DOIP_COMMON_EID_LEN] = bytes[0..eid_offset].try_into().ok()?;

        Some(Self { eid })
    }
}
