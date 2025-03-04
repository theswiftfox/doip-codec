use doip_definitions::{definitions::DOIP_HEADER_LEN, header::PayloadType, DoipMessage};
use heapless::Vec;

use crate::{
    doip_message::{
        header::HeaderCodec,
        payload::{
            alive_check_request::AliveCheckRequestCodec as AlivChecReqCodec,
            alive_check_response::AliveCheckResponseCodec as AlivChecResCodec,
            diagnostic_message::DiagnosticMessageCodec as DiagMsgCodec,
            diagnostic_message_ack::DiagnosticMessageAckCodec as DiagMsgAckCodec,
            diagnostic_message_nack::DiagnosticMessageNackCodec as DiagMsgNackCodec,
            entity_status_request::EntityStatusRequestCodec as EntStatReqCodec,
            entity_status_response::EntityStatusResponseCodec as EntStatResCodec,
            generic_nack::GenericNackCodec,
            power_information_request::PowerInformationRequestCodec as PowInfoReqCodec,
            power_information_response::PowerInformationResponseCodec as PowInfoResCodec,
            routing_activation_request::RoutingActivationRequestCodec as RoutActReqCodec,
            routing_activation_response::RoutingActivationResponseCodec as RoutActResCodec,
            vehicle_announcement_message::VehicleAnnouncementMessageCodec as VehAnnMsgCodec,
            vehicle_identification_request::VehicleIdentificationRequestCodec as VehIDReqCodec,
            vehicle_identification_request_eid::VehicleIdentificationRequestEidCodec as VehIDReqEidCodec,
            vehicle_identification_request_vin::VehicleIdentificationRequestVinCodec as VehIDReqVinCodec,
        },
    },
    error::DecodeError,
    Decoder, DoipCodec,
};

impl<const N: usize> Decoder<N> for DoipCodec<N> {
    type Item = DoipMessage<N>;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN {
            return Err(DecodeError::TooShort);
        }

        let mut h_codec = HeaderCodec {};

        let header = h_codec.decode(src)?.expect("Should never return Ok(None)");

        let payload = match header.payload_type {
            PayloadType::GenericNack => GenericNackCodec {}.decode(src)?,
            PayloadType::VehicleIdentificationRequest => VehIDReqCodec {}.decode(src)?,
            PayloadType::VehicleIdentificationRequestEid => VehIDReqEidCodec {}.decode(src)?,
            PayloadType::VehicleIdentificationRequestVin => VehIDReqVinCodec {}.decode(src)?,
            PayloadType::VehicleAnnouncementMessage => VehAnnMsgCodec {}.decode(src)?,
            PayloadType::RoutingActivationRequest => RoutActReqCodec {}.decode(src)?,
            PayloadType::RoutingActivationResponse => RoutActResCodec {}.decode(src)?,
            PayloadType::AliveCheckRequest => AlivChecReqCodec {}.decode(src)?,
            PayloadType::AliveCheckResponse => AlivChecResCodec {}.decode(src)?,
            PayloadType::EntityStatusRequest => EntStatReqCodec {}.decode(src)?,
            PayloadType::EntityStatusResponse => EntStatResCodec {}.decode(src)?,
            PayloadType::PowerInformationRequest => PowInfoReqCodec {}.decode(src)?,
            PayloadType::PowerInformationResponse => PowInfoResCodec {}.decode(src)?,
            PayloadType::DiagnosticMessage => DiagMsgCodec {}.decode(src)?,
            PayloadType::DiagnosticMessageAck => DiagMsgAckCodec {}.decode(src)?,
            PayloadType::DiagnosticMessageNack => DiagMsgNackCodec {}.decode(src)?,
        }
        .expect("Should never fail, this means header has been mutated during runtime");

        Ok(Some(DoipMessage { header, payload }))
    }
}