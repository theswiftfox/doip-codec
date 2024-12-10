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

    pub fn parse_from_bytes(mut src: Vec<u8>) -> Result<Vec<DoipMessage>, ParseError> {
        let mut messages: Vec<DoipMessage> = Vec::<DoipMessage>::new();

        if src.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        while !src.is_empty() {
            if src.len() < DOIP_HEADER_LEN {
                break;
            }

            let protocol_version = match DoipVersion::from_u8(src[DOIP_VERSION_OFFSET]) {
                Some(v) => v,
                None => return Err(ParseError::InvalidProtocolVersion),
            };

            match !protocol_version.to_u8() == src[DOIP_INV_VERSION_OFFSET] {
                true => {}
                false => return Err(ParseError::FailedProtocolCheck),
            };

            let payload_type = match PayloadType::from_bytes(
                &src[DOIP_TYPE_OFFSET..(DOIP_TYPE_OFFSET + DOIP_TYPE_LEN)],
            ) {
                Ok(p) => p,
                Err(err) => return Err(ParseError::PayloadParseError(err)),
            };

            let payload_length = u32::from_be_bytes([
                src[DOIP_LENGTH_OFFSET],
                src[DOIP_LENGTH_OFFSET + 1],
                src[DOIP_LENGTH_OFFSET + 2],
                src[DOIP_LENGTH_OFFSET + 3],
            ]) as usize;

            let payload_data = &src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + payload_length];

            let payload: Box<dyn DoipPayload> = match payload_type {
                PayloadType::GenericNack => {
                    // Assuming GenericNack implements DoipPayload
                    match GenericNack::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::VehicleIdentificationRequest => {
                    match VehicleIdentificationRequest::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::VehicleIdentificationRequestEid => {
                    match VehicleIdentificationRequestEid::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::VehicleIdentificationRequestVin => {
                    match VehicleIdentificationRequestVin::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::VehicleAnnouncementMessage => {
                    match VehicleAnnouncementMessage::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::RoutingActivationRequest => {
                    match RoutingActivationRequest::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::RoutingActivationResponse => {
                    match RoutingActivationResponse::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::AliveCheckRequest => match AliveCheckRequest::from_bytes(payload_data)
                {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParseError(err)),
                },
                PayloadType::AliveCheckResponse => {
                    match AliveCheckResponse::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::EntityStatusRequest => {
                    match EntityStatusRequest::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::EntityStatusResponse => {
                    match EntityStatusResponse::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::PowerInformationRequest => {
                    match PowerInformationRequest::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::PowerInformationResponse => {
                    match PowerInformationResponse::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::DiagnosticMessage => match DiagnosticMessage::from_bytes(payload_data)
                {
                    Ok(p) => Box::new(p),
                    Err(err) => return Err(ParseError::PayloadParseError(err)),
                },
                PayloadType::DiagnosticMessageAck => {
                    match DiagnosticMessageAck::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
                PayloadType::DiagnosticMessageNack => {
                    match DiagnosticMessageNack::from_bytes(payload_data) {
                        Ok(p) => Box::new(p),
                        Err(err) => return Err(ParseError::PayloadParseError(err)),
                    }
                }
            };

            // Create the DoipMessage with the payload
            let message = DoipMessage {
                header: DoipHeader::new(protocol_version, &*payload),
                payload,
            };

            messages.push(message);
            src.drain(0..(payload_length + 8));
        }

        Ok(messages)
    }
}
