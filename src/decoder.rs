use doip_definitions::{definitions::DOIP_HEADER_LEN, header::PayloadType, message::DoipMessage};
use tokio_util::bytes::Buf;

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

impl Decoder for DoipCodec {
    type Item = DoipMessage;
    type Error = DecodeError;

    fn decode_from_bytes(&mut self, src: &[u8]) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN {
            return Ok(None);
        }

        let mut h_codec = HeaderCodec {};

        let header = h_codec
            .decode_from_bytes(src)?
            .expect("Should never return Ok(None)");

        let payload = match header.payload_type {
            PayloadType::GenericNack => GenericNackCodec {}.decode_from_bytes(src)?,
            PayloadType::VehicleIdentificationRequest => VehIDReqCodec {}.decode_from_bytes(src)?,
            PayloadType::VehicleIdentificationRequestEid => {
                VehIDReqEidCodec {}.decode_from_bytes(src)?
            }
            PayloadType::VehicleIdentificationRequestVin => {
                VehIDReqVinCodec {}.decode_from_bytes(src)?
            }
            PayloadType::VehicleAnnouncementMessage => VehAnnMsgCodec {}.decode_from_bytes(src)?,
            PayloadType::RoutingActivationRequest => RoutActReqCodec {}.decode_from_bytes(src)?,
            PayloadType::RoutingActivationResponse => RoutActResCodec {}.decode_from_bytes(src)?,
            PayloadType::AliveCheckRequest => AlivChecReqCodec {}.decode_from_bytes(src)?,
            PayloadType::AliveCheckResponse => AlivChecResCodec {}.decode_from_bytes(src)?,
            PayloadType::EntityStatusRequest => EntStatReqCodec {}.decode_from_bytes(src)?,
            PayloadType::EntityStatusResponse => EntStatResCodec {}.decode_from_bytes(src)?,
            PayloadType::PowerInformationRequest => PowInfoReqCodec {}.decode_from_bytes(src)?,
            PayloadType::PowerInformationResponse => PowInfoResCodec {}.decode_from_bytes(src)?,
            PayloadType::DiagnosticMessage => DiagMsgCodec {}.decode_from_bytes(src)?,
            PayloadType::DiagnosticMessageAck => DiagMsgAckCodec {}.decode_from_bytes(src)?,
            PayloadType::DiagnosticMessageNack => DiagMsgNackCodec {}.decode_from_bytes(src)?,
        }
        .expect("Should never fail, this means header has been mutated during runtime");

        Ok(Some(DoipMessage { header, payload }))
    }
}

impl tokio_util::codec::Decoder for DoipCodec {
    type Item = DoipMessage;
    type Error = DecodeError;

    fn decode(
        &mut self,
        src: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let decoded = DoipCodec {}.decode_from_bytes(src);

        if let Err(DecodeError::TooShort) = decoded {
            return Ok(None);
        }

        let decoded = decoded?.inspect(|item| {
            src.advance(item.header.payload_length as usize + DOIP_HEADER_LEN);
        });

        Ok(decoded)
    }
}
