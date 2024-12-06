use crate::doip::definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN};

use super::payload::{DoipPayload, PayloadType};

#[derive(Clone, Debug)]
pub struct DiagnosticMessage {
    pub source_address: [u8; 2],
    pub target_address: [u8; 2],
    pub message: Vec<u8>,
}

impl DoipPayload for DiagnosticMessage {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessage
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&self.target_address);
        bytes.extend_from_slice(&self.message);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN;

        if bytes.len() < min_length {
            return None;
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            bytes[0..source_address_offset].try_into().ok()?;

        let target_address_offset = source_address_offset + DOIP_DIAG_COMMON_TARGET_LEN;
        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] = bytes
            [source_address_offset..target_address_offset]
            .try_into()
            .ok()?;

        let message = bytes[target_address_offset..].to_vec();

        Some(Self {
            source_address,
            target_address,
            message,
        })
    }
}
