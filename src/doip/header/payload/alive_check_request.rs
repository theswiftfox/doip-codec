use super::payload::{DoipPayload, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct AliveCheckRequest {}

impl DoipPayload for AliveCheckRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::AliveCheckRequest
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn from_bytes(_bytes: &[u8]) -> Option<Self> {
        Some(AliveCheckRequest {})
    }
}
