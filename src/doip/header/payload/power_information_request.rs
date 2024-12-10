use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct PowerInformationRequest {}

impl DoipPayload for PowerInformationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::PowerInformationRequest
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
        payload::{DoipPayload, PayloadType},
        power_information_request::PowerInformationRequest,
    };

    #[test]
    fn test_payload_type() {
        let request = PowerInformationRequest {};
        assert_eq!(request.payload_type(), PayloadType::PowerInformationRequest);
    }

    #[test]
    fn test_to_bytes() {
        let request = PowerInformationRequest {};
        assert_eq!(request.to_bytes(), vec![]);
    }

    #[test]
    fn test_from_bytes_ok() {
        let bytes = PowerInformationRequest {}.to_bytes();
        let request = PowerInformationRequest::from_bytes(&bytes);

        assert!(
            request.is_ok(),
            "Expected PowerInformationRequest, recieved an Error."
        );
    }
}
