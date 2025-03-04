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

    fn encode(&mut self, item: DoipPayload<N>, dst: &mut Vec<u8, N>) -> Result<(), Self::Error> {
        match item {
            DoipPayload::GenericNack(generic_nack) => {
                GenericNackCodec {}.encode(generic_nack, dst)?;
            }
            DoipPayload::VehicleIdentificationRequest(vehicle_identification_request) => {
                VehicleIdentificationRequestCodec {}.encode(vehicle_identification_request, dst)?;
            }
            DoipPayload::VehicleIdentificationRequestEid(vehicle_identification_request_eid) => {
                VehicleIdentificationRequestEidCodec {}
                    .encode(vehicle_identification_request_eid, dst)?;
            }
            DoipPayload::VehicleIdentificationRequestVin(vehicle_identification_request_vin) => {
                VehicleIdentificationRequestVinCodec {}
                    .encode(vehicle_identification_request_vin, dst)?;
            }
            DoipPayload::VehicleAnnouncementMessage(vehicle_announcement_message) => {
                VehicleAnnouncementMessageCodec {}.encode(vehicle_announcement_message, dst)?;
            }
            DoipPayload::RoutingActivationRequest(routing_activation_request) => {
                RoutingActivationRequestCodec {}.encode(routing_activation_request, dst)?;
            }
            DoipPayload::RoutingActivationResponse(routing_activation_response) => {
                RoutingActivationResponseCodec {}.encode(routing_activation_response, dst)?;
            }
            DoipPayload::AliveCheckRequest(alive_check_request) => {
                AliveCheckRequestCodec {}.encode(alive_check_request, dst)?;
            }
            DoipPayload::AliveCheckResponse(alive_check_response) => {
                AliveCheckResponseCodec {}.encode(alive_check_response, dst)?;
            }
            DoipPayload::EntityStatusRequest(entity_status_request) => {
                EntityStatusRequestCodec {}.encode(entity_status_request, dst)?;
            }
            DoipPayload::EntityStatusResponse(entity_status_response) => {
                EntityStatusResponseCodec {}.encode(entity_status_response, dst)?;
            }
            DoipPayload::PowerInformationRequest(power_information_request) => {
                PowerInformationRequestCodec {}.encode(power_information_request, dst)?;
            }
            DoipPayload::PowerInformationResponse(power_information_response) => {
                PowerInformationResponseCodec {}.encode(power_information_response, dst)?;
            }
            DoipPayload::DiagnosticMessage(diagnostic_message) => {
                DiagnosticMessageCodec {}.encode(diagnostic_message, dst)?;
            }
            DoipPayload::DiagnosticMessageAck(diagnostic_message_ack) => {
                DiagnosticMessageAckCodec {}.encode(diagnostic_message_ack, dst)?;
            }
            DoipPayload::DiagnosticMessageNack(diagnostic_message_nack) => {
                DiagnosticMessageNackCodec {}.encode(diagnostic_message_nack, dst)?;
            }
        };
        Ok(())
    }
}
