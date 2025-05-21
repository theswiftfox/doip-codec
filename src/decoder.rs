use doip_definitions::{definitions::DOIP_HEADER_LEN, header::PayloadType, message::DoipMessage};
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

    fn from_bytes(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN {
            return Err(DecodeError::TooShort);
        }

        let mut h_codec = HeaderCodec {};

        let header = h_codec
            .from_bytes(src)?
            .expect("Should never return Ok(None)");

        let payload = match header.payload_type {
            PayloadType::GenericNack => GenericNackCodec {}.from_bytes(src)?,
            PayloadType::VehicleIdentificationRequest => VehIDReqCodec {}.from_bytes(src)?,
            PayloadType::VehicleIdentificationRequestEid => VehIDReqEidCodec {}.from_bytes(src)?,
            PayloadType::VehicleIdentificationRequestVin => VehIDReqVinCodec {}.from_bytes(src)?,
            PayloadType::VehicleAnnouncementMessage => VehAnnMsgCodec {}.from_bytes(src)?,
            PayloadType::RoutingActivationRequest => RoutActReqCodec {}.from_bytes(src)?,
            PayloadType::RoutingActivationResponse => RoutActResCodec {}.from_bytes(src)?,
            PayloadType::AliveCheckRequest => AlivChecReqCodec {}.from_bytes(src)?,
            PayloadType::AliveCheckResponse => AlivChecResCodec {}.from_bytes(src)?,
            PayloadType::EntityStatusRequest => EntStatReqCodec {}.from_bytes(src)?,
            PayloadType::EntityStatusResponse => EntStatResCodec {}.from_bytes(src)?,
            PayloadType::PowerInformationRequest => PowInfoReqCodec {}.from_bytes(src)?,
            PayloadType::PowerInformationResponse => PowInfoResCodec {}.from_bytes(src)?,
            PayloadType::DiagnosticMessage => DiagMsgCodec {}.from_bytes(src)?,
            PayloadType::DiagnosticMessageAck => DiagMsgAckCodec {}.from_bytes(src)?,
            PayloadType::DiagnosticMessageNack => DiagMsgNackCodec {}.from_bytes(src)?,
        }
        .expect("Should never fail, this means header has been mutated during runtime");

        Ok(Some(DoipMessage { header, payload }))
    }
}

impl<const N: usize> tokio_util::codec::Decoder for DoipCodec<N> {
    type Item = DoipMessage<N>;
    type Error = DecodeError;

    fn decode(
        &mut self,
        src: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let mut heapless_src = heapless::Vec::<u8, N>::new();
        heapless_src
            .extend_from_slice(&src)
            .map_err(|_| DecodeError::BufferTooSmall)?;

        let decoder = DoipCodec {}.from_bytes(&mut heapless_src)?;

        Ok(decoder)
    }
}
