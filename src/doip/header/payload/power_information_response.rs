use crate::doip::{definitions::DOIP_POWER_MODE_LEN, message::power_mode::PowerMode};

use super::payload::{DoipPayload, PayloadType};

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

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_POWER_MODE_LEN;

        if bytes.len() < min_length {
            return None;
        }

        let power_mode_offset = DOIP_POWER_MODE_LEN;
        let power_mode = match &bytes[power_mode_offset] {
            0x00 => PowerMode::NotReady,
            0x01 => PowerMode::Ready,
            0x02 => PowerMode::NotSupported,
            _ => return None,
        };

        Some(Self { power_mode })
    }
}
