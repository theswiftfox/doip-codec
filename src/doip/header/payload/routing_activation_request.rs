use thiserror::Error;

use crate::doip::{
    definitions::{
        DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN, DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN,
        DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V1,
    },
    message::activation_type::ActivationType,
};

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct RoutingActivationRequest {
    pub source_address: [u8; 2],
    pub activation_type: ActivationType,
    pub buffer: [u8; 4],
}

impl DoipPayload for RoutingActivationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::RoutingActivationRequest
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&[self.activation_type as u8]);
        bytes.extend_from_slice(&self.buffer);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN
            + DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V1
            + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::RoutingActivationRequestError(
                RoutingActivationRequestError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN;
        let source_address: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::RoutingActivationRequestError(
                        RoutingActivationRequestError::InvalidIndexRange,
                    ))
                }
            };

        let activation_type_offset =
            source_address_offset + DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V1;

        let activation_type = match &bytes[activation_type_offset] {
            0x00 => ActivationType::Default,
            0x01 => ActivationType::WwhObd,
            0x02 => ActivationType::CentralSecurity,
            _ => {
                return Err(PayloadError::RoutingActivationRequestError(
                    RoutingActivationRequestError::ActivationType,
                ))
            }
        };

        let buffer_offset = activation_type_offset + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN;
        let buffer: [u8; DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN] =
            match bytes[activation_type_offset..buffer_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::RoutingActivationRequestError(
                        RoutingActivationRequestError::InvalidIndexRange,
                    ))
                }
            };

        Ok(Self {
            source_address,
            activation_type,
            buffer,
        })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RoutingActivationRequestError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
    #[error("activation type not supported")]
    ActivationType,
}
