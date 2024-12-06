use std::fmt::Debug;

use crate::doip::definitions::{DOIP_ALIVE_CHECK_REQUEST, DOIP_ALIVE_CHECK_RESPONSE, DOIP_DIAGNOSTIC_MESSAGE, DOIP_DIAGNOSTIC_MESSAGE_ACK, DOIP_DIAGNOSTIC_MESSAGE_NACK, DOIP_ENTITY_STATUS_REQUEST, DOIP_ENTITY_STATUS_RESPONSE, DOIP_GENERIC_NACK, DOIP_POWER_INFORMATION_REQUEST, DOIP_POWER_INFORMATION_RESPONSE, DOIP_ROUTING_ACTIVATION_REQUEST, DOIP_ROUTING_ACTIVATION_RESPONSE, DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE, DOIP_VEHICLE_IDENTIFICATION_REQ, DOIP_VEHICLE_IDENTIFICATION_REQ_EID, DOIP_VEHICLE_IDENTIFICATION_REQ_VIN};

pub trait DoipPayload: Debug + Send {
    fn payload_type(&self) -> PayloadType;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
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

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let bytes: [u8; 2] = [bytes[0], bytes[1]];
        let value = u16::from_be_bytes(bytes);

        match value {
            DOIP_GENERIC_NACK => Some(PayloadType::GenericNack),
            DOIP_VEHICLE_IDENTIFICATION_REQ => Some(PayloadType::VehicleIdentificationRequest),
            DOIP_VEHICLE_IDENTIFICATION_REQ_EID => {
                Some(PayloadType::VehicleIdentificationRequestEid)
            }
            DOIP_VEHICLE_IDENTIFICATION_REQ_VIN => {
                Some(PayloadType::VehicleIdentificationRequestVin)
            }
            DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE => Some(PayloadType::VehicleAnnouncementMessage),
            DOIP_ROUTING_ACTIVATION_REQUEST => Some(PayloadType::RoutingActivationRequest),
            DOIP_ROUTING_ACTIVATION_RESPONSE => Some(PayloadType::RoutingActivationResponse),
            DOIP_ALIVE_CHECK_REQUEST => Some(PayloadType::AliveCheckRequest),
            DOIP_ALIVE_CHECK_RESPONSE => Some(PayloadType::AliveCheckResponse),
            DOIP_ENTITY_STATUS_REQUEST => Some(PayloadType::EntityStatusRequest),
            DOIP_ENTITY_STATUS_RESPONSE => Some(PayloadType::EntityStatusResponse),
            DOIP_POWER_INFORMATION_REQUEST => Some(PayloadType::PowerInformationRequest),
            DOIP_POWER_INFORMATION_RESPONSE => Some(PayloadType::PowerInformationResponse),
            DOIP_DIAGNOSTIC_MESSAGE => Some(PayloadType::DiagnosticMessage),
            DOIP_DIAGNOSTIC_MESSAGE_ACK => Some(PayloadType::DiagnosticMessageAck),
            DOIP_DIAGNOSTIC_MESSAGE_NACK => Some(PayloadType::DiagnosticMessageNack),
            _ => None,
        }
    }

}
