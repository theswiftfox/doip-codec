use doip_definitions::{header::PayloadType, DoipMessage};
use tokio_util::bytes::BytesMut;

use crate::{
    doip_message::{HeaderCodec, PayloadCodec},
    error::EncodeError,
    DoipCodec, Encoder,
};

impl Encoder<DoipMessage<'static>> for DoipCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: DoipMessage,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let mut header_encoder = HeaderCodec {};

        match item.header.payload_type {
            PayloadType::GenericNack => todo!(),
            PayloadType::VehicleIdentificationRequest => todo!(),
            PayloadType::VehicleIdentificationRequestEid => todo!(),
            PayloadType::VehicleIdentificationRequestVin => todo!(),
            PayloadType::VehicleAnnouncementMessage => todo!(),
            PayloadType::RoutingActivationRequest => todo!(),
            PayloadType::RoutingActivationResponse => todo!(),
            PayloadType::AliveCheckRequest => todo!(),
            PayloadType::AliveCheckResponse => todo!(),
            PayloadType::EntityStatusRequest => todo!(),
            PayloadType::EntityStatusResponse => todo!(),
            PayloadType::PowerInformationRequest => todo!(),
            PayloadType::PowerInformationResponse => todo!(),
            PayloadType::DiagnosticMessage => todo!(),
            PayloadType::DiagnosticMessageAck => todo!(),
            PayloadType::DiagnosticMessageNack => todo!(),
        };

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{AliveCheckRequest, DoipPayload},
        DoipMessage,
    };
    use tokio_util::bytes::BytesMut;

    use crate::{DoipCodec, Encoder};

    #[test]
    fn test_encode_single_message_success() {
        let mut encoder = DoipCodec {};
        let mut dst = BytesMut::new();

        let item = DoipMessage {
            header: DoipHeader {
                protocol_version: ProtocolVersion::Iso13400_2012,
                inverse_protocol_version: 0xfd,
                payload_type: PayloadType::AliveCheckRequest,
                payload_length: 0u32,
            },
            payload: DoipPayload::AliveCheckRequest(AliveCheckRequest {}),
        };

        let bytes = encoder.encode(item, &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(*dst, [0x02, 0xfd, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00])
    }
}
