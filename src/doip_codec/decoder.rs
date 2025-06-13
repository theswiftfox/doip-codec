use crate::DoipCodec;
use crate::{Error, Result};
use doip_definitions::definitions::DOIP_HEADER_LEN;
use doip_definitions::header::{DoipHeader, PayloadType};
use doip_definitions::message::DoipMessage;
use doip_definitions::payload::{
    AliveCheckRequest, AliveCheckResponse, DiagnosticMessage, DiagnosticMessageAck,
    DiagnosticMessageNack, DoipPayload, EntityStatusRequest, EntityStatusResponse, GenericNack,
    PowerInformationRequest, PowerInformationResponse, RoutingActivationRequest,
    RoutingActivationResponse, VehicleAnnouncementMessage, VehicleIdentificationRequest,
    VehicleIdentificationRequestEid, VehicleIdentificationRequestVin,
};
#[cfg(feature = "std")]
use tokio_util::bytes::Buf;

// region:      --- no_std
#[cfg(not(feature = "std"))]
pub trait Decoder<const N: usize> {
    /// The type of decoded frames
    type Item;

    /// The type of unrecoverable frame decoding errors.
    ///
    /// If an individual message is ill-formed but can be ignored without interfering with the
    /// processing of future messages, it may be more useful to report the failure as an Item.
    type Error: From<Error>;

    /// Attempts to decode a frame from the provided buffer of bytes.
    fn decode(&mut self, src: &mut heapless::Vec<u8, N>) -> Result<Option<Self::Item>>;
}

#[cfg(not(feature = "std"))]
impl<const N: usize> Decoder<N> for DoipCodec<N> {
    type Item = DoipMessage<N>;
    type Error = Error;

    fn decode(&mut self, src: &mut heapless::Vec<u8, N>) -> Result<Option<Self::Item>> {
        // If the source has less than the number of bytes in a DoIP header, there is not enough data.
        if src.len() < DOIP_HEADER_LEN {
            return Ok(None);
        };

        let header_slice: [u8; DOIP_HEADER_LEN] = src[0..DOIP_HEADER_LEN].try_into()?;

        // Extract the header from the first eight bytes.
        let header = DoipHeader::try_from(header_slice)?;

        // If the source has less than the number of bytes in a DoIP header and the length of the payload, there is not enough data.
        if src.len() < DOIP_HEADER_LEN + (header.payload_length as usize) {
            return Ok(None);
        };

        let slice: &[u8] = &src[DOIP_HEADER_LEN..(header.payload_length as usize)];

        let payload = match header.payload_type {
            PayloadType::GenericNack => DoipPayload::GenericNack(GenericNack::try_from(slice)?),
            PayloadType::VehicleIdentificationRequest => {
                DoipPayload::VehicleIdentificationRequest(VehicleIdentificationRequest::from(slice))
            }
            PayloadType::VehicleIdentificationRequestEid => {
                DoipPayload::VehicleIdentificationRequestEid(
                    VehicleIdentificationRequestEid::try_from(slice)?,
                )
            }
            PayloadType::VehicleIdentificationRequestVin => {
                DoipPayload::VehicleIdentificationRequestVin(
                    VehicleIdentificationRequestVin::try_from(slice)?,
                )
            }
            PayloadType::VehicleAnnouncementMessage => DoipPayload::VehicleAnnouncementMessage(
                VehicleAnnouncementMessage::try_from(slice)?,
            ),
            PayloadType::RoutingActivationRequest => {
                DoipPayload::RoutingActivationRequest(RoutingActivationRequest::try_from(slice)?)
            }
            PayloadType::RoutingActivationResponse => {
                DoipPayload::RoutingActivationResponse(RoutingActivationResponse::try_from(slice)?)
            }
            PayloadType::AliveCheckRequest => {
                DoipPayload::AliveCheckRequest(AliveCheckRequest::from(slice))
            }
            PayloadType::AliveCheckResponse => {
                DoipPayload::AliveCheckResponse(AliveCheckResponse::try_from(slice)?)
            }
            PayloadType::EntityStatusRequest => {
                DoipPayload::EntityStatusRequest(EntityStatusRequest::from(slice))
            }
            PayloadType::EntityStatusResponse => {
                DoipPayload::EntityStatusResponse(EntityStatusResponse::try_from(slice)?)
            }
            PayloadType::PowerInformationRequest => {
                DoipPayload::PowerInformationRequest(PowerInformationRequest::from(slice))
            }
            PayloadType::PowerInformationResponse => {
                DoipPayload::PowerInformationResponse(PowerInformationResponse::try_from(slice)?)
            }
            PayloadType::DiagnosticMessage => {
                DoipPayload::DiagnosticMessage(DiagnosticMessage::try_from(slice)?)
            }
            PayloadType::DiagnosticMessageAck => {
                DoipPayload::DiagnosticMessageAck(DiagnosticMessageAck::try_from(slice)?)
            }
            PayloadType::DiagnosticMessageNack => {
                DoipPayload::DiagnosticMessageNack(DiagnosticMessageNack::try_from(slice)?)
            }
        };

        let mut new_vec = heapless::Vec::<u8, N>::new();
        new_vec.extend_from_slice(&src[DOIP_HEADER_LEN + (header.payload_length as usize)..])?;
        *src = new_vec;

        Ok(Some(DoipMessage { header, payload }))
    }
}

// endregion:   --- no_std

// region:      --- std

#[cfg(feature = "std")]
impl tokio_util::codec::Decoder for DoipCodec {
    type Item = DoipMessage;
    type Error = Error;

    fn decode(&mut self, src: &mut tokio_util::bytes::BytesMut) -> Result<Option<Self::Item>> {
        // If the source has less than the number of bytes in a DoIP header, there is not enough data.
        if src.len() < DOIP_HEADER_LEN {
            return Ok(None);
        };

        let header_slice: [u8; DOIP_HEADER_LEN] = src[0..DOIP_HEADER_LEN].try_into()?;

        // Extract the header from the first eight bytes.
        let header = DoipHeader::try_from(header_slice)?;

        // If the source has less than the number of bytes in a DoIP header and the length of the payload, there is not enough data.
        if src.len() < DOIP_HEADER_LEN + (header.payload_length as usize) {
            return Ok(None);
        };

        let slice: &[u8] =
            &src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + (header.payload_length as usize)];

        let payload = match header.payload_type {
            PayloadType::GenericNack => DoipPayload::GenericNack(GenericNack::try_from(slice)?),
            PayloadType::VehicleIdentificationRequest => {
                DoipPayload::VehicleIdentificationRequest(VehicleIdentificationRequest::from(slice))
            }
            PayloadType::VehicleIdentificationRequestEid => {
                DoipPayload::VehicleIdentificationRequestEid(
                    VehicleIdentificationRequestEid::try_from(slice)?,
                )
            }
            PayloadType::VehicleIdentificationRequestVin => {
                DoipPayload::VehicleIdentificationRequestVin(
                    VehicleIdentificationRequestVin::try_from(slice)?,
                )
            }
            PayloadType::VehicleAnnouncementMessage => DoipPayload::VehicleAnnouncementMessage(
                VehicleAnnouncementMessage::try_from(slice)?,
            ),
            PayloadType::RoutingActivationRequest => {
                DoipPayload::RoutingActivationRequest(RoutingActivationRequest::try_from(slice)?)
            }
            PayloadType::RoutingActivationResponse => {
                DoipPayload::RoutingActivationResponse(RoutingActivationResponse::try_from(slice)?)
            }
            PayloadType::AliveCheckRequest => {
                DoipPayload::AliveCheckRequest(AliveCheckRequest::from(slice))
            }
            PayloadType::AliveCheckResponse => {
                DoipPayload::AliveCheckResponse(AliveCheckResponse::try_from(slice)?)
            }
            PayloadType::EntityStatusRequest => {
                DoipPayload::EntityStatusRequest(EntityStatusRequest::from(slice))
            }
            PayloadType::EntityStatusResponse => {
                DoipPayload::EntityStatusResponse(EntityStatusResponse::try_from(slice)?)
            }
            PayloadType::PowerInformationRequest => {
                DoipPayload::PowerInformationRequest(PowerInformationRequest::from(slice))
            }
            PayloadType::PowerInformationResponse => {
                DoipPayload::PowerInformationResponse(PowerInformationResponse::try_from(slice)?)
            }
            PayloadType::DiagnosticMessage => {
                DoipPayload::DiagnosticMessage(DiagnosticMessage::try_from(slice)?)
            }
            PayloadType::DiagnosticMessageAck => {
                DoipPayload::DiagnosticMessageAck(DiagnosticMessageAck::try_from(slice)?)
            }
            PayloadType::DiagnosticMessageNack => {
                DoipPayload::DiagnosticMessageNack(DiagnosticMessageNack::try_from(slice)?)
            }
        };

        let cnt = DOIP_HEADER_LEN + (header.payload_length as usize);
        let advance_length = if src.remaining() >= cnt {
            cnt
        } else {
            src.len()
        };
        src.advance(advance_length);

        Ok(Some(DoipMessage { header, payload }))
    }
}

// endregion:   --- std

#[cfg(all(test, feature = "std"))]
mod tests {
    use tokio_util::codec::Decoder;

    #[test]
    fn test_decode() {
        let payload = vec![
            0x02, 0xfd, 0x80, 0x01, 0x00, 0x00, 0x00, 0x0b, 0x11, 0x06, 0x0f, 0x0d, 0x6a, 0xf0,
            0x00, 0x00, 0x00, 0x00, 0x01,
        ];
        let mut codec = super::DoipCodec {};
        let mut bytes = tokio_util::bytes::BytesMut::from(payload.as_slice());
        let result = codec.decode(&mut bytes);
        assert!(result.is_ok());

        let mut bytes_incomplete = tokio_util::bytes::BytesMut::from(&payload[..12]);
        let result_incomplete = codec.decode(&mut bytes_incomplete);
        assert!(result_incomplete.is_ok());
    }
}
