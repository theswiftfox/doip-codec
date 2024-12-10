use thiserror::Error;

use crate::doip::{
    definitions::{
        DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN, DOIP_ROUTING_ACTIVATION_RES_CODE_LEN,
        DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN, DOIP_ROUTING_ACTIVATION_RES_ISO_LEN,
        DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN,
    },
    message::activation_code::ActivationCode,
};

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct RoutingActivationResponse {
    pub logical_address: [u8; 2],
    pub source_address: [u8; 2],
    pub activation_code: ActivationCode,
    pub buffer: [u8; 4],
}

impl DoipPayload for RoutingActivationResponse {
    fn payload_type(&self) -> PayloadType {
        PayloadType::RoutingActivationResponse
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.logical_address);
        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&[self.activation_code as u8]);
        bytes.extend_from_slice(&self.buffer);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN
            + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN
            + DOIP_ROUTING_ACTIVATION_RES_CODE_LEN
            + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::RoutingActivationResponseError(
                RoutingActivationResponseError::InvalidLength,
            ));
        }

        let logical_address_offset = DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN;
        let logical_address: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN] =
            match bytes[0..logical_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::RoutingActivationResponseError(
                        RoutingActivationResponseError::InvalidIndexRange,
                    ))
                }
            };

        let source_address_offset = logical_address_offset + DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN;
        let source_address: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN] =
            match bytes[logical_address_offset..source_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::RoutingActivationResponseError(
                        RoutingActivationResponseError::InvalidIndexRange,
                    ))
                }
            };

        let activation_code_offset = source_address_offset;

        let activation_code = match &bytes[activation_code_offset] {
            0x00 => ActivationCode::DeniedUnknownSourceAddress,
            0x01 => ActivationCode::DeniedTCPSocketsFull,
            0x02 => ActivationCode::DeniedTCPSocketAlreadyConnected,
            0x03 => ActivationCode::DeniedSourceIsAlreadyActive,
            0x04 => ActivationCode::DeniedMissingAuthentication,
            0x05 => ActivationCode::DeniedRejectedConfirmation,
            0x06 => ActivationCode::DeniedUnsupportedRoutingActivationType,
            0x07 => ActivationCode::DeniedRequestEncryptedTLSConnection,
            0x08 => ActivationCode::ReservedByIso13400_08,
            0x09 => ActivationCode::ReservedByIso13400_09,
            0x0A => ActivationCode::ReservedByIso13400_0A,
            0x0B => ActivationCode::ReservedByIso13400_0B,
            0x0C => ActivationCode::ReservedByIso13400_0C,
            0x0D => ActivationCode::ReservedByIso13400_0D,
            0x0E => ActivationCode::ReservedByIso13400_0E,
            0x0F => ActivationCode::ReservedByIso13400_0F,
            0x10 => ActivationCode::SuccessfullyActivated,
            0x11 => ActivationCode::ActivatedConfirmationRequired,
            _ => {
                return Err(PayloadError::RoutingActivationResponseError(
                    RoutingActivationResponseError::InvalidActivationCode,
                ))
            }
        };

        let buffer_offset = activation_code_offset
            + DOIP_ROUTING_ACTIVATION_RES_CODE_LEN
            + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN;

        let buffer: [u8; DOIP_ROUTING_ACTIVATION_RES_ISO_LEN] = match bytes
            [(activation_code_offset + DOIP_ROUTING_ACTIVATION_RES_CODE_LEN)..buffer_offset]
            .try_into()
        {
            Ok(arr) => arr,
            Err(_) => {
                return Err(PayloadError::RoutingActivationResponseError(
                    RoutingActivationResponseError::InvalidIndexRange,
                ))
            }
        };

        Ok(Self {
            logical_address,
            source_address,
            activation_code,
            buffer,
        })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RoutingActivationResponseError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
    #[error("activation code not supported")]
    InvalidActivationCode,
}
