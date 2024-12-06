use crate::doip::definitions::DOIP_DIAG_COMMON_SOURCE_LEN;

use super::payload::{DoipPayload, PayloadType};

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

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN;

        if bytes.len() < min_length {
            return None;
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            bytes[0..source_address_offset].try_into().ok()?;

        Some(Self { source_address })
    }
}
