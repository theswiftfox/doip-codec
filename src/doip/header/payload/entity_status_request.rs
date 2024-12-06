use super::payload::{DoipPayload, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct EntityStatusRequest {}

impl DoipPayload for EntityStatusRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::EntityStatusRequest
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn from_bytes(_bytes: &[u8]) -> Option<Self> {
      Some(Self {})
    }
}
