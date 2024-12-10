use thiserror::Error;

use crate::doip::{
    definitions::{
        DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN, DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN, DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN,
    },
    message::node_type::NodeType,
};

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct EntityStatusResponse {
    pub node_type: NodeType,
    pub max_concurrent_sockets: [u8; 1],
    pub currently_open_sockets: [u8; 1],
    pub max_data_size: [u8; 4],
}

impl DoipPayload for EntityStatusResponse {
    fn payload_type(&self) -> PayloadType {
        PayloadType::EntityStatusResponse
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&[self.node_type as u8]);
        bytes.extend_from_slice(&self.max_concurrent_sockets);
        bytes.extend_from_slice(&self.currently_open_sockets);
        bytes.extend_from_slice(&self.max_data_size);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::EntityStatusResponseError(
                EntityStatusResponseError::InvalidLength,
            ));
        }

        let node_type_offset = DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN;
        let node_type = match &bytes[0] {
            0x00 => NodeType::DoipGateway,
            0x01 => NodeType::DoipNode,
            _ => {
                return Err(PayloadError::EntityStatusResponseError(
                    EntityStatusResponseError::InvalidNodeType,
                ))
            }
        };

        let max_concurrent_sockets_offset = node_type_offset + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN;
        let max_concurrent_sockets: [u8; DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN] =
            match bytes[node_type_offset..max_concurrent_sockets_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::EntityStatusResponseError(
                        EntityStatusResponseError::InvalidIndexRange,
                    ))
                }
            };

        let currently_open_sockets_offset =
            max_concurrent_sockets_offset + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN;
        let currently_open_sockets: [u8; DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN] =
            match bytes[max_concurrent_sockets_offset..currently_open_sockets_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::EntityStatusResponseError(
                        EntityStatusResponseError::InvalidIndexRange,
                    ))
                }
            };

        let max_data_size_offset =
            currently_open_sockets_offset + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN;
        let max_data_size: [u8; DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN] =
            match bytes[currently_open_sockets_offset..max_data_size_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::EntityStatusResponseError(
                        EntityStatusResponseError::InvalidIndexRange,
                    ))
                }
            };

        Ok(Self {
            node_type,
            max_concurrent_sockets,
            currently_open_sockets,
            max_data_size,
        })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum EntityStatusResponseError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
    #[error("invalid node type")]
    InvalidNodeType,
}
