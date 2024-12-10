use super::{
    payload::payload::{DoipPayload, PayloadType},
    version::DoipVersion,
};

#[derive(Debug)]
pub struct DoipHeader {
    pub protocol_version: DoipVersion,
    pub inverse_protocol_version: u8,
    pub payload_type: PayloadType,
    pub payload_length: u32,
}

impl DoipHeader {
    pub fn new(protocol_version: DoipVersion, payload: &dyn DoipPayload) -> Self {
        Self {
            protocol_version,
            inverse_protocol_version: !protocol_version.to_u8(),
            payload_type: payload.payload_type(),
            payload_length: payload.to_bytes().len() as u32,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let protocol_version_bytes: Vec<u8> = [self.protocol_version.to_u8()].to_vec();
        let inverse_protocol_version_bytes: Vec<u8> = [self.inverse_protocol_version].to_vec();
        let payload_type_bytes: Vec<u8> = self.payload_type.to_bytes();
        let payload_length_bytes: Vec<u8> = self.payload_length.to_be_bytes().to_vec();

        bytes.extend_from_slice(&protocol_version_bytes);
        bytes.extend_from_slice(&inverse_protocol_version_bytes);
        bytes.extend_from_slice(&payload_type_bytes);
        bytes.extend_from_slice(&payload_length_bytes);

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::doip::header::{
        header::DoipHeader, payload::vehicle_identification_request::VehicleIdentificationRequest,
        version::DoipVersion,
    };

    #[test]
    fn test_to_bytes() {
        let payload = Box::new(VehicleIdentificationRequest {});
        let payload_ref = &*payload;
        let header = DoipHeader::new(DoipVersion::Iso13400_2012, payload_ref);

        assert_eq!(
            header.to_bytes(),
            vec![0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]
        );
    }
}
