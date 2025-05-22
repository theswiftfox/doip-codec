use doip_definitions::{
    definitions::{
        DOIP_HEADER_LEN, DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN,
        DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V2, DOIP_ROUTING_ACTIVATION_REQ_LEN,
        DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN, DOIP_ROUTING_ACTIVATION_REQ_TYPE_OFFSET,
    },
    payload::{ActivationType, DoipPayload, RoutingActivationRequest},
};

use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};

#[derive(Debug)]
pub struct RoutingActivationRequestCodec;

impl Encoder<RoutingActivationRequest> for RoutingActivationRequestCodec {
    type Error = EncodeError;

    fn to_bytes(&mut self, item: RoutingActivationRequest, dst: &mut Vec<u8>) -> Result<(), Self::Error> {
        let RoutingActivationRequest {
            source_address,
            activation_type,
            buffer,
        } = item;

        dst.extend_from_slice(&source_address);

        let activation_type_bytes = activation_type.to_bytes();
        dst.extend_from_slice(activation_type_bytes);

        dst.extend_from_slice(&buffer);

        Ok(())
    }
}

impl ToBytes for ActivationType {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            ActivationType::Default => &[ActivationType::Default as u8],
            ActivationType::WwhObd => &[ActivationType::WwhObd as u8],
            ActivationType::CentralSecurity => &[ActivationType::CentralSecurity as u8],
        }
    }
}

impl Decoder for RoutingActivationRequestCodec {
    type Item = DoipPayload;

    type Error = DecodeError;

    fn decode_from_bytes(&mut self, src: &[u8]) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_ROUTING_ACTIVATION_REQ_LEN {
            return Err(DecodeError::TooShort);
        }

        let source_address = src
            [DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let activation_type_bytes =
            &src[DOIP_ROUTING_ACTIVATION_REQ_TYPE_OFFSET..=DOIP_ROUTING_ACTIVATION_REQ_TYPE_OFFSET];

        let activation_type = ActivationType::from_bytes(activation_type_bytes)
            .ok_or(DecodeError::InvalidActivationType)?;

        let buffer = src[DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V2
            ..DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V2 + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let item = RoutingActivationRequest {
            source_address,
            activation_type,
            buffer,
        };

        Ok(Some(DoipPayload::RoutingActivationRequest(item)))
    }
}

impl FromBytes for ActivationType {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        match val {
            v if v == ActivationType::Default as u8 => Some(ActivationType::Default),
            v if v == ActivationType::WwhObd as u8 => Some(ActivationType::WwhObd),
            v if v == ActivationType::CentralSecurity as u8 => {
                Some(ActivationType::CentralSecurity)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        message::DoipMessage,
        payload::{ActivationType, DoipPayload, RoutingActivationRequest},
    };

    use crate::{Decoder, DoipCodec, Encoder, FromBytes, ToBytes};

    static SUCCESS_ROOT: DoipMessage = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::RoutingActivationRequest,
            payload_length: 7u32,
        },
        payload: DoipPayload::RoutingActivationRequest(RoutingActivationRequest {
            source_address: [0x00, 0x00],
            activation_type: ActivationType::Default,
            buffer: [0x00, 0x00, 0x00, 0x00],
        }),
    };

    #[test]
    fn test_activation_type_to_bytes() {
        let bytes = ActivationType::Default.to_bytes();
        assert_eq!(bytes, &[0x00]);
        let bytes = ActivationType::WwhObd.to_bytes();
        assert_eq!(bytes, &[0x01]);
        let bytes = ActivationType::CentralSecurity.to_bytes();
        assert_eq!(bytes, &[0x02]);
    }
    #[test]
    fn test_activation_type_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = ActivationType::from_bytes(bytes);

            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationType::Default);
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationType::WwhObd);
                }
                0x02 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationType::CentralSecurity);
                }
                _ => {
                    assert!(proto.is_none());
                }
            }
        }
    }

    #[test]
    fn test_decode_routing_activation_request_success() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let _ = codec.to_bytes(SUCCESS_ROOT.clone(), &mut dst);
        let msg = codec.decode_from_bytes(&mut dst);

        assert!(msg.is_ok());
        let opt = msg.unwrap();

        assert!(opt.is_some());
        let res = opt.unwrap();

        assert_eq!(res, SUCCESS_ROOT);
    }

    #[test]
    fn test_encode_routing_activation_request_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let bytes = encoder.to_bytes(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [
                0x02, 0xfd, 0x00, 0x05, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00
            ]
        );
    }

    #[test]
    fn test_decode_routing_activation_request_invalid_activation_type() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let bytes = &[
            0x02, 0xfd, 0x00, 0x05, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00,
            0x00,
        ];
        dst.extend_from_slice(bytes);
        let msg = codec.decode_from_bytes(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_routing_activation_request_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let bytes = &[
            0x02, 0xfd, 0x00, 0x05, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x42, 0x00, 0x00,
        ];
        dst.extend_from_slice(bytes);
        let msg = codec.decode_from_bytes(&mut dst);

        assert!(msg.is_err());
    }
}
