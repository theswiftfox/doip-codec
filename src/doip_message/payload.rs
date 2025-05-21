use alive_check_request::AliveCheckRequestCodec;
use alive_check_response::AliveCheckResponseCodec;
use diagnostic_message::DiagnosticMessageCodec;
use diagnostic_message_ack::DiagnosticMessageAckCodec;
use diagnostic_message_nack::DiagnosticMessageNackCodec;
use doip_definitions::payload::DoipPayload;
use entity_status_request::EntityStatusRequestCodec;
use entity_status_response::EntityStatusResponseCodec;
use generic_nack::GenericNackCodec;
use heapless::Vec;
use power_information_request::PowerInformationRequestCodec;
use power_information_response::PowerInformationResponseCodec;
use routing_activation_request::RoutingActivationRequestCodec;
use routing_activation_response::RoutingActivationResponseCodec;
use vehicle_announcement_message::VehicleAnnouncementMessageCodec;
use vehicle_identification_request::VehicleIdentificationRequestCodec;
use vehicle_identification_request_eid::VehicleIdentificationRequestEidCodec;
use vehicle_identification_request_vin::VehicleIdentificationRequestVinCodec;

use crate::{EncodeError, Encoder};

pub mod alive_check_request;
pub mod alive_check_response;
pub mod diagnostic_message;
pub mod diagnostic_message_ack;
pub mod diagnostic_message_nack;
pub mod entity_status_request;
pub mod entity_status_response;
pub mod generic_nack;
pub mod power_information_request;
pub mod power_information_response;
pub mod routing_activation_request;
pub mod routing_activation_response;
pub mod vehicle_announcement_message;
pub mod vehicle_identification_request;
pub mod vehicle_identification_request_eid;
pub mod vehicle_identification_request_vin;

#[derive(Debug)]
pub struct PayloadCodec;

impl<const N: usize> Encoder<DoipPayload<N>, N> for PayloadCodec {
    type Error = EncodeError;

    fn to_bytes(&mut self, item: DoipPayload<N>, dst: &mut Vec<u8, N>) -> Result<(), Self::Error> {
        match item {
            DoipPayload::GenericNack(generic_nack) => {
                GenericNackCodec {}.to_bytes(generic_nack, dst)?;
            }
            DoipPayload::VehicleIdentificationRequest(vehicle_identification_request) => {
                VehicleIdentificationRequestCodec {}.to_bytes(vehicle_identification_request, dst)?;
            }
            DoipPayload::VehicleIdentificationRequestEid(vehicle_identification_request_eid) => {
                VehicleIdentificationRequestEidCodec {}
                    .to_bytes(vehicle_identification_request_eid, dst)?;
            }
            DoipPayload::VehicleIdentificationRequestVin(vehicle_identification_request_vin) => {
                VehicleIdentificationRequestVinCodec {}
                    .to_bytes(vehicle_identification_request_vin, dst)?;
            }
            DoipPayload::VehicleAnnouncementMessage(vehicle_announcement_message) => {
                VehicleAnnouncementMessageCodec {}.to_bytes(vehicle_announcement_message, dst)?;
            }
            DoipPayload::RoutingActivationRequest(routing_activation_request) => {
                RoutingActivationRequestCodec {}.to_bytes(routing_activation_request, dst)?;
            }
            DoipPayload::RoutingActivationResponse(routing_activation_response) => {
                RoutingActivationResponseCodec {}.to_bytes(routing_activation_response, dst)?;
            }
            DoipPayload::AliveCheckRequest(alive_check_request) => {
                AliveCheckRequestCodec {}.to_bytes(alive_check_request, dst)?;
            }
            DoipPayload::AliveCheckResponse(alive_check_response) => {
                AliveCheckResponseCodec {}.to_bytes(alive_check_response, dst)?;
            }
            DoipPayload::EntityStatusRequest(entity_status_request) => {
                EntityStatusRequestCodec {}.to_bytes(entity_status_request, dst)?;
            }
            DoipPayload::EntityStatusResponse(entity_status_response) => {
                EntityStatusResponseCodec {}.to_bytes(entity_status_response, dst)?;
            }
            DoipPayload::PowerInformationRequest(power_information_request) => {
                PowerInformationRequestCodec {}.to_bytes(power_information_request, dst)?;
            }
            DoipPayload::PowerInformationResponse(power_information_response) => {
                PowerInformationResponseCodec {}.to_bytes(power_information_response, dst)?;
            }
            DoipPayload::DiagnosticMessage(diagnostic_message) => {
                DiagnosticMessageCodec {}.to_bytes(diagnostic_message, dst)?;
            }
            DoipPayload::DiagnosticMessageAck(diagnostic_message_ack) => {
                DiagnosticMessageAckCodec {}.to_bytes(diagnostic_message_ack, dst)?;
            }
            DoipPayload::DiagnosticMessageNack(diagnostic_message_nack) => {
                DiagnosticMessageNackCodec {}.to_bytes(diagnostic_message_nack, dst)?;
            }
        }
        Ok(())
    }
}
