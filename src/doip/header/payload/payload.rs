use std::fmt::Debug;

use thiserror::Error;

use crate::doip::definitions::{
    DOIP_ALIVE_CHECK_REQUEST, DOIP_ALIVE_CHECK_RESPONSE, DOIP_DIAGNOSTIC_MESSAGE,
    DOIP_DIAGNOSTIC_MESSAGE_ACK, DOIP_DIAGNOSTIC_MESSAGE_NACK, DOIP_ENTITY_STATUS_REQUEST,
    DOIP_ENTITY_STATUS_RESPONSE, DOIP_GENERIC_NACK, DOIP_POWER_INFORMATION_REQUEST,
    DOIP_POWER_INFORMATION_RESPONSE, DOIP_ROUTING_ACTIVATION_REQUEST,
    DOIP_ROUTING_ACTIVATION_RESPONSE, DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,
    DOIP_VEHICLE_IDENTIFICATION_REQ, DOIP_VEHICLE_IDENTIFICATION_REQ_EID,
    DOIP_VEHICLE_IDENTIFICATION_REQ_VIN,
};

use super::{
    alive_check_response::AliveCheckResponseParseError, diagnostic_message::DiagnosticMessageError,
    diagnostic_message_ack::DiagnosticMessageAckError,
    diagnostic_message_nack::DiagnosticMessageNackParseError,
    entity_status_response::EntityStatusResponseError, generic_nack::GenericNackError,
    power_information_response::PowerInformationResponseError,
    routing_activation_request::RoutingActivationRequestError,
    routing_activation_response::RoutingActivationResponseError,
    vehicle_announcement_message::VehicleAnnouncementMessageError,
    vehicle_identification_request_eid::VehicleIdentificationRequestEidError,
    vehicle_identification_request_vin::VehicleIdentificationRequestVinError,
};

pub trait DoipPayload: Debug + Send {
    fn payload_type(&self) -> PayloadType;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError>
    where
        Self: Sized;
}

#[derive(Error, Debug, PartialEq)]
pub enum PayloadError {
    #[error("alive check payload parse failure")]
    AliveCheckResponseParseError(#[from] AliveCheckResponseParseError),
    #[error("diuagnostic message negative acknolwedgement payload parse failure")]
    DiagnosticMessageNackParseError(#[from] DiagnosticMessageNackParseError),
    #[error("diagnostic message acknowledgement payload parse failure")]
    DiagnosticMessageAckError(#[from] DiagnosticMessageAckError),
    #[error("diagnostic message payload parse failure")]
    DiagnosticMessageError(#[from] DiagnosticMessageError),
    #[error("entity status response payload parse failure")]
    EntityStatusResponseError(#[from] EntityStatusResponseError),
    #[error("generic nack payload parse failure")]
    GenericNackError(#[from] GenericNackError),
    #[error("power information response payload parse failure")]
    PowerInformationResponseError(#[from] PowerInformationResponseError),
    #[error("routing activation request payload parse failure")]
    RoutingActivationRequestError(#[from] RoutingActivationRequestError),
    #[error("routing activation response payload parse failure")]
    RoutingActivationResponseError(#[from] RoutingActivationResponseError),
    #[error("vehicle announcement message payload parse failure")]
    VehicleAnnouncementMessageError(#[from] VehicleAnnouncementMessageError),
    #[error("vehicle identification request with eid payload parse failure")]
    VehicleIdentificationRequestEidError(#[from] VehicleIdentificationRequestEidError),
    #[error("vehicle identification request with vin payload parse failure")]
    VehicleIdentificationRequestVinError(#[from] VehicleIdentificationRequestVinError),
    #[error("invalid payload type")]
    InvalidPayloadType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum PayloadType {
    GenericNack = DOIP_GENERIC_NACK,
    VehicleIdentificationRequest = DOIP_VEHICLE_IDENTIFICATION_REQ,
    VehicleIdentificationRequestEid = DOIP_VEHICLE_IDENTIFICATION_REQ_EID,
    VehicleIdentificationRequestVin = DOIP_VEHICLE_IDENTIFICATION_REQ_VIN,
    VehicleAnnouncementMessage = DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,
    RoutingActivationRequest = DOIP_ROUTING_ACTIVATION_REQUEST,
    RoutingActivationResponse = DOIP_ROUTING_ACTIVATION_RESPONSE,
    AliveCheckRequest = DOIP_ALIVE_CHECK_REQUEST,
    AliveCheckResponse = DOIP_ALIVE_CHECK_RESPONSE,
    EntityStatusRequest = DOIP_ENTITY_STATUS_REQUEST,
    EntityStatusResponse = DOIP_ENTITY_STATUS_RESPONSE,
    PowerInformationRequest = DOIP_POWER_INFORMATION_REQUEST,
    PowerInformationResponse = DOIP_POWER_INFORMATION_RESPONSE,
    DiagnosticMessage = DOIP_DIAGNOSTIC_MESSAGE,
    DiagnosticMessageAck = DOIP_DIAGNOSTIC_MESSAGE_ACK,
    DiagnosticMessageNack = DOIP_DIAGNOSTIC_MESSAGE_NACK,
}

impl DoipPayload for PayloadType {
    fn payload_type(&self) -> PayloadType {
        *self
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = *self as u16;
        value.to_be_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<PayloadType, PayloadError> {
        let bytes: [u8; 2] = [bytes[0], bytes[1]];
        let value = u16::from_be_bytes(bytes);

        match value {
            DOIP_GENERIC_NACK => Ok(PayloadType::GenericNack),
            DOIP_VEHICLE_IDENTIFICATION_REQ => Ok(PayloadType::VehicleIdentificationRequest),
            DOIP_VEHICLE_IDENTIFICATION_REQ_EID => Ok(PayloadType::VehicleIdentificationRequestEid),
            DOIP_VEHICLE_IDENTIFICATION_REQ_VIN => Ok(PayloadType::VehicleIdentificationRequestVin),
            DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE => Ok(PayloadType::VehicleAnnouncementMessage),
            DOIP_ROUTING_ACTIVATION_REQUEST => Ok(PayloadType::RoutingActivationRequest),
            DOIP_ROUTING_ACTIVATION_RESPONSE => Ok(PayloadType::RoutingActivationResponse),
            DOIP_ALIVE_CHECK_REQUEST => Ok(PayloadType::AliveCheckRequest),
            DOIP_ALIVE_CHECK_RESPONSE => Ok(PayloadType::AliveCheckResponse),
            DOIP_ENTITY_STATUS_REQUEST => Ok(PayloadType::EntityStatusRequest),
            DOIP_ENTITY_STATUS_RESPONSE => Ok(PayloadType::EntityStatusResponse),
            DOIP_POWER_INFORMATION_REQUEST => Ok(PayloadType::PowerInformationRequest),
            DOIP_POWER_INFORMATION_RESPONSE => Ok(PayloadType::PowerInformationResponse),
            DOIP_DIAGNOSTIC_MESSAGE => Ok(PayloadType::DiagnosticMessage),
            DOIP_DIAGNOSTIC_MESSAGE_ACK => Ok(PayloadType::DiagnosticMessageAck),
            DOIP_DIAGNOSTIC_MESSAGE_NACK => Ok(PayloadType::DiagnosticMessageNack),
            _ => Err(PayloadError::InvalidPayloadType),
        }
    }
}
