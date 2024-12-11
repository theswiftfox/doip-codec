use crate::{
    doip::{
        definitions::{
            DOIP_HEADER_LEN, DOIP_INV_VERSION_OFFSET, DOIP_LENGTH_OFFSET, DOIP_TYPE_LEN,
            DOIP_TYPE_OFFSET, DOIP_VERSION_OFFSET,
        },
        header::{
            header::DoipHeader,
            payload::{
                alive_check_request::AliveCheckRequest,
                alive_check_response::AliveCheckResponse,
                diagnostic_message::DiagnosticMessage,
                diagnostic_message_ack::DiagnosticMessageAck,
                diagnostic_message_nack::DiagnosticMessageNack,
                entity_status_request::EntityStatusRequest,
                entity_status_response::EntityStatusResponse,
                generic_nack::GenericNack,
                payload::{DoipPayload, PayloadType},
                power_information_request::PowerInformationRequest,
                power_information_response::PowerInformationResponse,
                routing_activation_request::RoutingActivationRequest,
                routing_activation_response::RoutingActivationResponse,
                vehicle_announcement_message::VehicleAnnouncementMessage,
                vehicle_identification_request::VehicleIdentificationRequest,
                vehicle_identification_request_eid::VehicleIdentificationRequestEid,
                vehicle_identification_request_vin::VehicleIdentificationRequestVin,
            },
            version::DoipVersion,
        },
    },
    error::ParseError,
};

#[derive(Debug)]
pub struct DoipMessage {
    pub header: DoipHeader,
    pub payload: Box<dyn DoipPayload>,
}

impl DoipMessage {
    pub fn new(protocol_version: DoipVersion, payload: Box<dyn DoipPayload>) -> Self {
        let payload_ref = &*payload;
        Self {
            header: DoipHeader::new(protocol_version, payload_ref),
            payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let header_bytes = self.header.to_bytes();
        let payload_bytes = self.payload.to_bytes();

        bytes.extend_from_slice(&header_bytes);
        bytes.extend_from_slice(&payload_bytes);

        bytes
    }

    pub fn parse_from_bytes(src: Vec<u8>) -> Result<DoipMessage, ParseError> {
        if src.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let proto_version_bytes: &u8 = match src.get(DOIP_VERSION_OFFSET) {
            Some(bytes) => bytes,
            None => return Err(ParseError::IndexFailure),
        };

        let protocol_version = match DoipVersion::from_u8(*proto_version_bytes) {
            Some(v) => v,
            None => return Err(ParseError::InvalidProtocolVersion),
        };

        let inv_proto_version_bytes: &u8 = match src.get(DOIP_INV_VERSION_OFFSET) {
            Some(bytes) => bytes,
            None => return Err(ParseError::IndexFailure),
        };

        match !protocol_version.to_u8() == *inv_proto_version_bytes {
            true => {}
            false => return Err(ParseError::FailedProtocolCheck),
        };

        let payload_type_bytes: &[u8] =
            match src.get(DOIP_TYPE_OFFSET..(DOIP_TYPE_OFFSET + DOIP_TYPE_LEN)) {
                Some(bytes) => bytes,
                None => return Err(ParseError::IndexFailure),
            };

        let payload_type = match PayloadType::from_bytes(payload_type_bytes) {
            Ok(p) => p,
            Err(err) => return Err(ParseError::PayloadParse(err)),
        };

        let payload_len_hh_byte: &u8 = match src.get(DOIP_LENGTH_OFFSET) {
            Some(bytes) => bytes,
            None => return Err(ParseError::IndexFailure),
        };

        let payload_len_hl_byte: &u8 = match src.get(DOIP_LENGTH_OFFSET + 1) {
            Some(bytes) => bytes,
            None => return Err(ParseError::IndexFailure),
        };

        let payload_len_lh_byte: &u8 = match src.get(DOIP_LENGTH_OFFSET + 2) {
            Some(bytes) => bytes,
            None => return Err(ParseError::IndexFailure),
        };

        let payload_len_ll_byte: &u8 = match src.get(DOIP_LENGTH_OFFSET + 3) {
            Some(bytes) => bytes,
            None => return Err(ParseError::IndexFailure),
        };

        let payload_length = u32::from_be_bytes([
            *payload_len_hh_byte,
            *payload_len_hl_byte,
            *payload_len_lh_byte,
            *payload_len_ll_byte,
        ]) as usize;

        let payload_data_bytes: &[u8] =
            match src.get(DOIP_HEADER_LEN..DOIP_HEADER_LEN + payload_length) {
                Some(bytes) => bytes,
                None => return Err(ParseError::IncompletePayload),
            };

        let payload: Box<dyn DoipPayload> = match payload_type {
            PayloadType::GenericNack => {
                // Assuming GenericNack implements DoipPayload
                match GenericNack::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::VehicleIdentificationRequest => {
                match VehicleIdentificationRequest::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::VehicleIdentificationRequestEid => {
                match VehicleIdentificationRequestEid::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::VehicleIdentificationRequestVin => {
                match VehicleIdentificationRequestVin::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::VehicleAnnouncementMessage => {
                match VehicleAnnouncementMessage::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::RoutingActivationRequest => {
                match RoutingActivationRequest::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::RoutingActivationResponse => {
                match RoutingActivationResponse::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::AliveCheckRequest => {
                match AliveCheckRequest::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::AliveCheckResponse => {
                match AliveCheckResponse::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::EntityStatusRequest => {
                match EntityStatusRequest::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::EntityStatusResponse => {
                match EntityStatusResponse::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::PowerInformationRequest => {
                match PowerInformationRequest::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::PowerInformationResponse => {
                match PowerInformationResponse::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::DiagnosticMessage => {
                match DiagnosticMessage::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::DiagnosticMessageAck => {
                match DiagnosticMessageAck::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
            PayloadType::DiagnosticMessageNack => {
                match DiagnosticMessageNack::from_bytes(payload_data_bytes) {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParse(err)),
                }
            }
        };

        // Create the DoipMessage with the payload
        let message = DoipMessage {
            header: DoipHeader::new(protocol_version, &*payload),
            payload,
        };

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip::header::payload::{
            payload::PayloadError, vehicle_identification_request::VehicleIdentificationRequest,
        },
        error::ParseError,
    };

    use super::DoipMessage;

    #[test]
    fn test_to_bytes() {
        let msg = DoipMessage::new(
            crate::doip::header::version::DoipVersion::Iso13400_2012,
            Box::new(VehicleIdentificationRequest {}),
        );

        let bytes = msg.to_bytes();

        assert_eq!(bytes, [0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00])
    }

    #[test]
    fn test_parse_from_bytes_ok() {
        let bytes = [0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let msg_raw = DoipMessage::parse_from_bytes(bytes.to_vec());

        assert!(msg_raw.is_ok(), "Expected msg to be ok.");

        let msg = msg_raw.unwrap();

        let comp = DoipMessage::new(
            crate::doip::header::version::DoipVersion::Iso13400_2012,
            Box::new(VehicleIdentificationRequest {}),
        );
        let msg_len_raw = (&*msg.payload).to_bytes().len();
        let comp_len_raw = (&*comp.payload).to_bytes().len();

        assert_eq!(msg.header, comp.header);
        assert_eq!(msg_len_raw, comp_len_raw);
    }

    #[test]
    fn test_parse_from_bytes_empty() {
        let bytes = [];
        let msg_raw = DoipMessage::parse_from_bytes(bytes.to_vec());

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::EmptyInput."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            ParseError::EmptyInput,
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_parse_from_bytes_invalid_protocol() {
        let bytes = [0x05, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let msg_raw = DoipMessage::parse_from_bytes(bytes.to_vec());

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::InvalidProtocolVersion."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            ParseError::InvalidProtocolVersion,
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_parse_from_bytes_failed_protocol_check() {
        let bytes = [0x02, 0xff, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let msg_raw = DoipMessage::parse_from_bytes(bytes.to_vec());

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::FailedProtocolCheck."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            ParseError::FailedProtocolCheck,
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_parse_from_bytes_payload_type_err() {
        let bytes = [0x02, 0xfd, 0x90, 0x01, 0x00, 0x00, 0x00, 0x00];
        let msg_raw = DoipMessage::parse_from_bytes(bytes.to_vec());

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::PayloadParseError(PayloadError::InvalidPayloadType)."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            ParseError::PayloadParse(PayloadError::InvalidPayloadType),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_parse_from_bytes_incomplete_data() {
        let bytes = [0x02, 0xfd, 0x40, 0x02, 0x00, 0x00, 0x00, 0x07, 0x00];
        let msg_raw = DoipMessage::parse_from_bytes(bytes.to_vec());

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::IncompletePayload."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            ParseError::IncompletePayload,
            "Unexpected error message: {}",
            error
        );
    }
}
