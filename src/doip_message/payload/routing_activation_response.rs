use doip_definitions::{
    definitions::{
        DOIP_HEADER_LEN,
        DOIP_ROUTING_ACTIVATION_RES_CODE_OFFSET, DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN,
        DOIP_ROUTING_ACTIVATION_RES_ENTITY_OFFSET, DOIP_ROUTING_ACTIVATION_RES_ISO_LEN,
        DOIP_ROUTING_ACTIVATION_RES_ISO_OFFSET, DOIP_ROUTING_ACTIVATION_RES_LEN,
        DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN,
    },
    payload::{ActivationCode, DoipPayload, RoutingActivationResponse},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};

#[derive(Debug)]
pub struct RoutingActivationResponseCodec;

impl<const N: usize> Encoder<RoutingActivationResponse, N> for RoutingActivationResponseCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: RoutingActivationResponse,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let RoutingActivationResponse {
            logical_address,
            source_address,
            activation_code,
            buffer,
        } = item;

        dst.extend_from_slice(&logical_address).map_err(|()| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&source_address).map_err(|()| EncodeError::BufferTooSmall)?;

        let activation_code_bytes = activation_code.to_bytes();
        dst.extend_from_slice(activation_code_bytes).map_err(|()| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&buffer).map_err(|()| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl ToBytes for ActivationCode {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            ActivationCode::DeniedUnknownSourceAddress => {
                &[ActivationCode::DeniedUnknownSourceAddress as u8]
            }
            ActivationCode::DeniedTCPSocketsFull => &[ActivationCode::DeniedTCPSocketsFull as u8],
            ActivationCode::DeniedTCPSocketAlreadyConnected => {
                &[ActivationCode::DeniedTCPSocketAlreadyConnected as u8]
            }
            ActivationCode::DeniedSourceIsAlreadyActive => {
                &[ActivationCode::DeniedSourceIsAlreadyActive as u8]
            }
            ActivationCode::DeniedMissingAuthentication => {
                &[ActivationCode::DeniedMissingAuthentication as u8]
            }
            ActivationCode::DeniedRejectedConfirmation => {
                &[ActivationCode::DeniedRejectedConfirmation as u8]
            }
            ActivationCode::DeniedUnsupportedRoutingActivationType => {
                &[ActivationCode::DeniedUnsupportedRoutingActivationType as u8]
            }
            ActivationCode::DeniedRequestEncryptedTLSConnection => {
                &[ActivationCode::DeniedRequestEncryptedTLSConnection as u8]
            }
            ActivationCode::ReservedByIso13400_08 => &[ActivationCode::ReservedByIso13400_08 as u8],
            ActivationCode::ReservedByIso13400_09 => &[ActivationCode::ReservedByIso13400_09 as u8],
            ActivationCode::ReservedByIso13400_0A => &[ActivationCode::ReservedByIso13400_0A as u8],
            ActivationCode::ReservedByIso13400_0B => &[ActivationCode::ReservedByIso13400_0B as u8],
            ActivationCode::ReservedByIso13400_0C => &[ActivationCode::ReservedByIso13400_0C as u8],
            ActivationCode::ReservedByIso13400_0D => &[ActivationCode::ReservedByIso13400_0D as u8],
            ActivationCode::ReservedByIso13400_0E => &[ActivationCode::ReservedByIso13400_0E as u8],
            ActivationCode::ReservedByIso13400_0F => &[ActivationCode::ReservedByIso13400_0F as u8],
            ActivationCode::SuccessfullyActivated => &[ActivationCode::SuccessfullyActivated as u8],
            ActivationCode::ActivatedConfirmationRequired => {
                &[ActivationCode::ActivatedConfirmationRequired as u8]
            }
        }
    }
}

impl<const N: usize> Decoder<N> for RoutingActivationResponseCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_ROUTING_ACTIVATION_RES_LEN {
            return Err(DecodeError::TooShort);
        }

        let logical_address = src
            [DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let source_address = src[DOIP_ROUTING_ACTIVATION_RES_ENTITY_OFFSET
            ..DOIP_ROUTING_ACTIVATION_RES_ENTITY_OFFSET + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let activation_code_bytes = &src[DOIP_ROUTING_ACTIVATION_RES_CODE_OFFSET..=DOIP_ROUTING_ACTIVATION_RES_CODE_OFFSET];
        let activation_code = ActivationCode::from_bytes(activation_code_bytes)
            .ok_or(DecodeError::InvalidActivationCode)?;

        let buffer = src[DOIP_ROUTING_ACTIVATION_RES_ISO_OFFSET
            ..DOIP_ROUTING_ACTIVATION_RES_ISO_OFFSET + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let item = RoutingActivationResponse {
            logical_address,
            source_address,
            activation_code,
            buffer,
        };

        Ok(Some(DoipPayload::RoutingActivationResponse(item)))
    }
}

impl FromBytes for ActivationCode {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        match val {
            v if v == ActivationCode::DeniedUnknownSourceAddress as u8 => {
                Some(ActivationCode::DeniedUnknownSourceAddress)
            }
            v if v == ActivationCode::DeniedTCPSocketsFull as u8 => {
                Some(ActivationCode::DeniedTCPSocketsFull)
            }
            v if v == ActivationCode::DeniedTCPSocketAlreadyConnected as u8 => {
                Some(ActivationCode::DeniedTCPSocketAlreadyConnected)
            }
            v if v == ActivationCode::DeniedSourceIsAlreadyActive as u8 => {
                Some(ActivationCode::DeniedSourceIsAlreadyActive)
            }
            v if v == ActivationCode::DeniedMissingAuthentication as u8 => {
                Some(ActivationCode::DeniedMissingAuthentication)
            }
            v if v == ActivationCode::DeniedRejectedConfirmation as u8 => {
                Some(ActivationCode::DeniedRejectedConfirmation)
            }
            v if v == ActivationCode::DeniedUnsupportedRoutingActivationType as u8 => {
                Some(ActivationCode::DeniedUnsupportedRoutingActivationType)
            }
            v if v == ActivationCode::DeniedRequestEncryptedTLSConnection as u8 => {
                Some(ActivationCode::DeniedRequestEncryptedTLSConnection)
            }
            v if v == ActivationCode::ReservedByIso13400_08 as u8 => {
                Some(ActivationCode::ReservedByIso13400_08)
            }
            v if v == ActivationCode::ReservedByIso13400_09 as u8 => {
                Some(ActivationCode::ReservedByIso13400_09)
            }
            v if v == ActivationCode::ReservedByIso13400_0A as u8 => {
                Some(ActivationCode::ReservedByIso13400_0A)
            }
            v if v == ActivationCode::ReservedByIso13400_0B as u8 => {
                Some(ActivationCode::ReservedByIso13400_0B)
            }
            v if v == ActivationCode::ReservedByIso13400_0C as u8 => {
                Some(ActivationCode::ReservedByIso13400_0C)
            }
            v if v == ActivationCode::ReservedByIso13400_0D as u8 => {
                Some(ActivationCode::ReservedByIso13400_0D)
            }
            v if v == ActivationCode::ReservedByIso13400_0E as u8 => {
                Some(ActivationCode::ReservedByIso13400_0E)
            }
            v if v == ActivationCode::ReservedByIso13400_0F as u8 => {
                Some(ActivationCode::ReservedByIso13400_0F)
            }
            v if v == ActivationCode::SuccessfullyActivated as u8 => {
                Some(ActivationCode::SuccessfullyActivated)
            }
            v if v == ActivationCode::ActivatedConfirmationRequired as u8 => {
                Some(ActivationCode::ActivatedConfirmationRequired)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{ActivationCode, DoipPayload, RoutingActivationResponse},
        DoipMessage,
    };
    use heapless::Vec;

    use crate::{DecodeError, Decoder, DoipCodec, Encoder, FromBytes, ToBytes};

    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::RoutingActivationResponse,
            payload_length: 9u32,
        },
        payload: DoipPayload::RoutingActivationResponse(RoutingActivationResponse {
            source_address: [0x00, 0x00],
            logical_address: [0x00, 0x00],
            activation_code: ActivationCode::SuccessfullyActivated,
            buffer: [0x00, 0x00, 0x00, 0x00],
        }),
    };

    #[test]
    fn test_activation_code_to_bytes() {
        let bytes = ActivationCode::DeniedUnknownSourceAddress.to_bytes();
        assert_eq!(bytes, &[0x00]);
        let bytes = ActivationCode::DeniedTCPSocketsFull.to_bytes();
        assert_eq!(bytes, &[0x01]);
        let bytes = ActivationCode::DeniedTCPSocketAlreadyConnected.to_bytes();
        assert_eq!(bytes, &[0x02]);
        let bytes = ActivationCode::DeniedSourceIsAlreadyActive.to_bytes();
        assert_eq!(bytes, &[0x03]);
        let bytes = ActivationCode::DeniedMissingAuthentication.to_bytes();
        assert_eq!(bytes, &[0x04]);
        let bytes = ActivationCode::DeniedRejectedConfirmation.to_bytes();
        assert_eq!(bytes, &[0x05]);
        let bytes = ActivationCode::DeniedUnsupportedRoutingActivationType.to_bytes();
        assert_eq!(bytes, &[0x06]);
        let bytes = ActivationCode::DeniedRequestEncryptedTLSConnection.to_bytes();
        assert_eq!(bytes, &[0x07]);
        let bytes = ActivationCode::ReservedByIso13400_08.to_bytes();
        assert_eq!(bytes, &[0x08]);
        let bytes = ActivationCode::ReservedByIso13400_09.to_bytes();
        assert_eq!(bytes, &[0x09]);
        let bytes = ActivationCode::ReservedByIso13400_0A.to_bytes();
        assert_eq!(bytes, &[0x0a]);
        let bytes = ActivationCode::ReservedByIso13400_0B.to_bytes();
        assert_eq!(bytes, &[0x0b]);
        let bytes = ActivationCode::ReservedByIso13400_0C.to_bytes();
        assert_eq!(bytes, &[0x0c]);
        let bytes = ActivationCode::ReservedByIso13400_0D.to_bytes();
        assert_eq!(bytes, &[0x0d]);
        let bytes = ActivationCode::ReservedByIso13400_0E.to_bytes();
        assert_eq!(bytes, &[0x0e]);
        let bytes = ActivationCode::ReservedByIso13400_0F.to_bytes();
        assert_eq!(bytes, &[0x0f]);
        let bytes = ActivationCode::SuccessfullyActivated.to_bytes();
        assert_eq!(bytes, &[0x10]);
        let bytes = ActivationCode::ActivatedConfirmationRequired.to_bytes();
        assert_eq!(bytes, &[0x11]);
    }

    #[test]
    fn test_activation_type_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = ActivationCode::from_bytes(bytes);

            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::DeniedUnknownSourceAddress)
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::DeniedTCPSocketsFull)
                }
                0x02 => {
                    assert!(proto.is_some());
                    assert_eq!(
                        proto.unwrap(),
                        ActivationCode::DeniedTCPSocketAlreadyConnected
                    )
                }
                0x03 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::DeniedSourceIsAlreadyActive)
                }
                0x04 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::DeniedMissingAuthentication)
                }
                0x05 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::DeniedRejectedConfirmation)
                }
                0x06 => {
                    assert!(proto.is_some());
                    assert_eq!(
                        proto.unwrap(),
                        ActivationCode::DeniedUnsupportedRoutingActivationType
                    )
                }
                0x07 => {
                    assert!(proto.is_some());
                    assert_eq!(
                        proto.unwrap(),
                        ActivationCode::DeniedRequestEncryptedTLSConnection
                    )
                }
                0x08 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::ReservedByIso13400_08)
                }
                0x09 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::ReservedByIso13400_09)
                }
                0x0a => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::ReservedByIso13400_0A)
                }
                0x0b => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::ReservedByIso13400_0B)
                }
                0x0c => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::ReservedByIso13400_0C)
                }
                0x0d => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::ReservedByIso13400_0D)
                }
                0x0e => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::ReservedByIso13400_0E)
                }
                0x0f => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::ReservedByIso13400_0F)
                }
                0x10 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActivationCode::SuccessfullyActivated)
                }
                0x11 => {
                    assert!(proto.is_some());
                    assert_eq!(
                        proto.unwrap(),
                        ActivationCode::ActivatedConfirmationRequired
                    )
                }
                _ => {
                    assert!(proto.is_none())
                }
            }
        }
    }

    #[test]
    fn test_encode_routing_activation_response_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [
                0x02, 0xfd, 0x00, 0x06, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
                0x00, 0x00, 0x00
            ]
        )
    }

    #[test]
    fn test_decode_routing_activation_response_success() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let _ = codec.encode(SUCCESS_ROOT.clone(), &mut dst);
        let msg = codec.decode(&mut dst);

        assert!(msg.is_ok());
        let opt = msg.unwrap();

        assert!(opt.is_some());
        let res = opt.unwrap();

        assert_eq!(res, SUCCESS_ROOT)
    }

    #[test]
    fn test_decode_routing_activation_response_invalid_activation_type() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x00, 0x06, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x42, 0x00,
            0x00, 0x00, 0x00,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_routing_activation_response_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x00, 0x06, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x10,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }
}
