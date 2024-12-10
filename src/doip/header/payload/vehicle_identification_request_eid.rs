use thiserror::Error;

use crate::doip::definitions::DOIP_COMMON_EID_LEN;

use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestEid {
    pub eid: [u8; DOIP_COMMON_EID_LEN],
}

impl DoipPayload for VehicleIdentificationRequestEid {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequestEid
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.eid.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_EID_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::VehicleIdentificationRequestEidError(
                VehicleIdentificationRequestEidError::InvalidLength,
            ));
        }

        let eid_offset = DOIP_COMMON_EID_LEN;
        let eid: [u8; DOIP_COMMON_EID_LEN] = match bytes[0..eid_offset].try_into() {
            Ok(arr) => arr,
            Err(_) => {
                return Err(PayloadError::VehicleIdentificationRequestEidError(
                    VehicleIdentificationRequestEidError::InvalidIndexRange,
                ))
            }
        };

        Ok(Self { eid })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum VehicleIdentificationRequestEidError {
    #[error("length of bytes is too short")]
    InvalidLength,
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}

#[cfg(test)]
mod tests {
    use crate::doip::{
        definitions::DOIP_COMMON_EID_LEN,
        header::payload::{
            payload::{DoipPayload, PayloadError, PayloadType},
            vehicle_identification_request_eid::{
                VehicleIdentificationRequestEid, VehicleIdentificationRequestEidError,
            },
        },
    };

    const DEFAULT_EID: [u8; DOIP_COMMON_EID_LEN] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];

    #[test]
    fn test_payload_type() {
        let request = VehicleIdentificationRequestEid { eid: DEFAULT_EID };
        assert_eq!(
            request.payload_type(),
            PayloadType::VehicleIdentificationRequestEid
        );
    }

    #[test]
    fn test_to_bytes() {
        let request = VehicleIdentificationRequestEid { eid: DEFAULT_EID };
        assert_eq!(request.to_bytes(), DEFAULT_EID.to_vec());
    }

    #[test]
    fn test_from_bytes_invalid_length() {
        let bytes = [0x00, 0x01, 0x02, 0x03, 0x04];
        let request = VehicleIdentificationRequestEid::from_bytes(&bytes);

        assert!(
            request.is_err(),
            "Expected to receive an VehicleIdentificationRequestEidParse::InvalidLength."
        );

        let error = request.unwrap_err();

        assert_eq!(
            error,
            PayloadError::VehicleIdentificationRequestEidError(
                VehicleIdentificationRequestEidError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let bytes = VehicleIdentificationRequestEid { eid: DEFAULT_EID }.to_bytes();
        let request = VehicleIdentificationRequestEid::from_bytes(&bytes);

        assert!(
            request.is_ok(),
            "Expected VehicleIdentificationRequestEid, recieved an Error."
        );
    }
}
