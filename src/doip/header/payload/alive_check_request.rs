use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct AliveCheckRequest {}

impl DoipPayload for AliveCheckRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::AliveCheckRequest
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Self, PayloadError> {
        Ok(AliveCheckRequest {})
    }
}

#[cfg(test)]
mod tests {
    use crate::doip::header::payload::{
        alive_check_request::AliveCheckRequest,
        payload::{DoipPayload, PayloadType},
    };

    #[test]
    fn test_payload_type() {
        let request = AliveCheckRequest {};
        assert_eq!(request.payload_type(), PayloadType::AliveCheckRequest);
    }

    #[test]
    fn test_to_bytes() {
        let request = AliveCheckRequest {};
        assert_eq!(request.to_bytes(), vec![]);
    }

    #[test]
    fn test_from_bytes_ok() {
        let bytes = AliveCheckRequest {}.to_bytes();
        let request = AliveCheckRequest::from_bytes(&bytes);

        assert!(
            request.is_ok(),
            "Expected AliveCheckRequest, recieved an Error."
        );
    }
}
