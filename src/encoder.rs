use doip_definitions::{header::PayloadType, payload::DoipPayload, DoipMessage};
use heapless::Vec;

use crate::{
    doip_message::{header::HeaderCodec, payload::PayloadCodec},
    error::EncodeError,
    DoipCodec, Encoder,
};

impl<const N: usize> Encoder<DoipMessage<N>, N> for DoipCodec<N> {
    type Error = EncodeError;

    fn to_bytes(&mut self, item: DoipMessage<N>, dst: &mut Vec<u8, N>) -> Result<(), Self::Error> {
        validate_payload_match(&item)?;

        let header_len = item.header.payload_length as usize;
        let () = HeaderCodec {}.to_bytes(item.header, dst)?;

        let before_len = dst.len();
        let () = PayloadCodec {}.to_bytes(item.payload, dst)?;
        let after_len = dst.len();

        validate_payload_length(header_len, after_len - before_len)?;

        Ok(())
    }
}

fn validate_payload_match<const N: usize>(item: &DoipMessage<N>) -> Result<(), EncodeError> {
    let valid = match item.payload {
        DoipPayload::GenericNack(_) => item.header.payload_type == PayloadType::GenericNack,
        DoipPayload::VehicleIdentificationRequest(_) => {
            item.header.payload_type == PayloadType::VehicleIdentificationRequest
        }
        DoipPayload::VehicleIdentificationRequestEid(_) => {
            item.header.payload_type == PayloadType::VehicleIdentificationRequestEid
        }
        DoipPayload::VehicleIdentificationRequestVin(_) => {
            item.header.payload_type == PayloadType::VehicleIdentificationRequestVin
        }
        DoipPayload::VehicleAnnouncementMessage(_) => {
            item.header.payload_type == PayloadType::VehicleAnnouncementMessage
        }
        DoipPayload::RoutingActivationRequest(_) => {
            item.header.payload_type == PayloadType::RoutingActivationRequest
        }
        DoipPayload::RoutingActivationResponse(_) => {
            item.header.payload_type == PayloadType::RoutingActivationResponse
        }
        DoipPayload::AliveCheckRequest(_) => {
            item.header.payload_type == PayloadType::AliveCheckRequest
        }
        DoipPayload::AliveCheckResponse(_) => {
            item.header.payload_type == PayloadType::AliveCheckResponse
        }
        DoipPayload::EntityStatusRequest(_) => {
            item.header.payload_type == PayloadType::EntityStatusRequest
        }
        DoipPayload::EntityStatusResponse(_) => {
            item.header.payload_type == PayloadType::EntityStatusResponse
        }
        DoipPayload::PowerInformationRequest(_) => {
            item.header.payload_type == PayloadType::PowerInformationRequest
        }
        DoipPayload::PowerInformationResponse(_) => {
            item.header.payload_type == PayloadType::PowerInformationResponse
        }
        DoipPayload::DiagnosticMessage(_) => {
            item.header.payload_type == PayloadType::DiagnosticMessage
        }
        DoipPayload::DiagnosticMessageAck(_) => {
            item.header.payload_type == PayloadType::DiagnosticMessageAck
        }
        DoipPayload::DiagnosticMessageNack(_) => {
            item.header.payload_type == PayloadType::DiagnosticMessageNack
        }
    };

    if valid {
        Ok(())
    } else {
        Err(EncodeError::PayloadTypeValidation)
    }
}

fn validate_payload_length(header_len: usize, length: usize) -> Result<(), EncodeError> {
    if header_len != length {
        return Err(EncodeError::PayloadLengthValidation);
    }
    Ok(())
}

#[cfg(feature = "std")]
impl<const N: usize> tokio_util::codec::Encoder<DoipMessage<N>> for DoipCodec<N> {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: DoipMessage<N>,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let mut heapless_dst = heapless::Vec::<u8, N>::new();
        heapless_dst
            .extend_from_slice(&dst)
            .map_err(|_| EncodeError::BufferTooSmall)?;
        DoipCodec {}.to_bytes(item, &mut heapless_dst)
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{AliveCheckRequest, DoipPayload, GenericNack, NackCode},
        DoipMessage,
    };

    use crate::encoder::validate_payload_length;

    use super::validate_payload_match;

    #[test]
    fn test_validate_payload_match() {
        let item_valid = DoipMessage {
            header: DoipHeader {
                protocol_version: ProtocolVersion::Iso13400_2012,
                inverse_protocol_version: 0xfd,
                payload_type: PayloadType::GenericNack,
                payload_length: 1u32,
            },
            payload: DoipPayload::<1>::GenericNack(GenericNack {
                nack_code: NackCode::OutOfMemory,
            }),
        };
        let valid = validate_payload_match(&item_valid);
        assert!(valid.is_ok());

        let item_invalid = DoipMessage {
            header: DoipHeader {
                protocol_version: ProtocolVersion::Iso13400_2012,
                inverse_protocol_version: 0xfd,
                payload_type: PayloadType::GenericNack,
                payload_length: 1u32,
            },
            payload: DoipPayload::<1>::AliveCheckRequest(AliveCheckRequest {}),
        };

        let invalid = validate_payload_match(&item_invalid);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_validate_payload_length() {
        let valid = validate_payload_length(1, 1);
        assert!(valid.is_ok());

        let invalid = validate_payload_length(1, 2);
        assert!(invalid.is_err());
    }
}
