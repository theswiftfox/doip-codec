use crate::doip::{definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_MESSAGE_NACK_CODE_LEN}, message::diagnostic_nack::DiagnosticNackNode};

use super::payload::{DoipPayload, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct DiagnosticMessageNack {
    pub source_address: [u8; 2],
    pub target_address: [u8; 2],
    pub nack_code: DiagnosticNackNode,
}

impl DoipPayload for DiagnosticMessageNack {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessageNack
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&self.target_address);
        bytes.extend_from_slice(&[self.nack_code as u8]);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_DIAG_COMMON_TARGET_LEN
            + DOIP_DIAG_MESSAGE_NACK_CODE_LEN;

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

        let nack_code_offset = target_address_offset;
        let nack_code = match &bytes[nack_code_offset] {
            0x00 => DiagnosticNackNode::ReservedByIso13400_00,
            0x01 => DiagnosticNackNode::ReservedByIso13400_01,
            0x02 => DiagnosticNackNode::InvalidSourceAddress,
            0x03 => DiagnosticNackNode::UnknownTargetAddress,
            0x04 => DiagnosticNackNode::DiagnosticMessageTooLarge,
            0x05 => DiagnosticNackNode::OutOfMemory,
            0x06 => DiagnosticNackNode::TargetUnreachable,
            0x07 => DiagnosticNackNode::UnknownNetwork,
            0x08 => DiagnosticNackNode::TransportProtocolError,
            _ => return None,
        };

        Some(Self {
            source_address,
            target_address,
            nack_code,
        })
    }
}
