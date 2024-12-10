use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct EntityStatusRequest {}

impl DoipPayload for EntityStatusRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::EntityStatusRequest
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Self, PayloadError> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use crate::doip::header::payload::{
        entity_status_request::EntityStatusRequest,
        payload::{DoipPayload, PayloadType},
    };

    #[test]
    fn test_payload_type() {
        let request = EntityStatusRequest {};
        assert_eq!(request.payload_type(), PayloadType::EntityStatusRequest);
    }

    #[test]
    fn test_to_bytes() {
        let request = EntityStatusRequest {};
        assert_eq!(request.to_bytes(), vec![]);
    }

    #[test]
    fn test_from_bytes_ok() {
        let bytes = EntityStatusRequest {}.to_bytes();
        let request = EntityStatusRequest::from_bytes(&bytes);

        assert!(
            request.is_ok(),
            "Expected EntityStatusRequest, recieved an Error."
        );
    }
}
