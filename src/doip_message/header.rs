use doip_definitions::{
    definitions::{
        DOIP_HEADER_LEN, DOIP_INV_VERSION_OFFSET, DOIP_LENGTH_LEN, DOIP_LENGTH_OFFSET,
        DOIP_TYPE_LEN, DOIP_TYPE_OFFSET, DOIP_VERSION_OFFSET,
    },
    header::{DoipHeader, PayloadType, ProtocolVersion},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};

#[derive(Debug)]
pub struct HeaderCodec;

impl<const N: usize> Encoder<DoipHeader, N> for HeaderCodec {
    type Error = EncodeError;

    fn encode(&mut self, item: DoipHeader, dst: &mut Vec<u8, N>) -> Result<(), Self::Error> {
        let DoipHeader {
            protocol_version,
            inverse_protocol_version,
            payload_type,
            payload_length,
        } = item;

        let protocol_version_bytes = protocol_version.to_bytes();
        let inverse_protocol_version_bytes: &[u8] = &[inverse_protocol_version];
        let payload_type_bytes: &[u8] = payload_type.to_bytes();
        let payload_length_bytes: &[u8] = &payload_length.to_be_bytes();

        if validate_protocol(protocol_version_bytes[0], inverse_protocol_version).is_none() {
            return Err(EncodeError::FailedProtocolValidation);
        }

        dst.extend_from_slice(protocol_version_bytes)
            .map_err(|()| EncodeError::BufferTooSmall)?;
        dst.extend_from_slice(inverse_protocol_version_bytes)
            .map_err(|()| EncodeError::BufferTooSmall)?;
        dst.extend_from_slice(payload_type_bytes)
            .map_err(|()| EncodeError::BufferTooSmall)?;
        dst.extend_from_slice(payload_length_bytes)
            .map_err(|()| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl ToBytes for ProtocolVersion {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            ProtocolVersion::ReservedVer => &[ProtocolVersion::ReservedVer as u8],
            ProtocolVersion::Iso13400_2010 => &[ProtocolVersion::Iso13400_2010 as u8],
            ProtocolVersion::Iso13400_2012 => &[ProtocolVersion::Iso13400_2012 as u8],
            ProtocolVersion::Iso13400_2019 => &[ProtocolVersion::Iso13400_2019 as u8],
            ProtocolVersion::Iso13400_2019Amd1 => &[ProtocolVersion::Iso13400_2019Amd1 as u8],
            ProtocolVersion::DefaultValue => &[ProtocolVersion::DefaultValue as u8],
        }
    }
}

impl ToBytes for PayloadType {
    fn to_bytes(self) -> &'static [u8] {
        static GENERIC_NACK_BYTES: [u8; 2] = (PayloadType::GenericNack as u16).to_be_bytes();
        static VEHICLE_IDENTIFICATION_REQUEST_BYTES: [u8; 2] =
            (PayloadType::VehicleIdentificationRequest as u16).to_be_bytes();
        static VEHICLE_IDENTIFICATION_REQUEST_EID_BYTES: [u8; 2] =
            (PayloadType::VehicleIdentificationRequestEid as u16).to_be_bytes();
        static VEHICLE_IDENTIFICATION_REQUEST_VIN_BYTES: [u8; 2] =
            (PayloadType::VehicleIdentificationRequestVin as u16).to_be_bytes();
        static VEHICLE_ANNOUNCEMENT_MESSAGE_BYTES: [u8; 2] =
            (PayloadType::VehicleAnnouncementMessage as u16).to_be_bytes();
        static ROUTING_ACTIVATION_REQUEST_BYTES: [u8; 2] =
            (PayloadType::RoutingActivationRequest as u16).to_be_bytes();
        static ROUTING_ACTIVATION_RESPONSE_BYTES: [u8; 2] =
            (PayloadType::RoutingActivationResponse as u16).to_be_bytes();
        static ALIVE_CHECK_REQUEST_BYTES: [u8; 2] =
            (PayloadType::AliveCheckRequest as u16).to_be_bytes();
        static ALIVE_CHECK_RESPONSE_BYTES: [u8; 2] =
            (PayloadType::AliveCheckResponse as u16).to_be_bytes();
        static ENTITY_STATUS_REQUEST_BYTES: [u8; 2] =
            (PayloadType::EntityStatusRequest as u16).to_be_bytes();
        static ENTITY_STATUS_RESPONSE_BYTES: [u8; 2] =
            (PayloadType::EntityStatusResponse as u16).to_be_bytes();
        static POWER_INFORMATION_REQUEST_BYTES: [u8; 2] =
            (PayloadType::PowerInformationRequest as u16).to_be_bytes();
        static POWER_INFORMATION_RESPONSE_BYTES: [u8; 2] =
            (PayloadType::PowerInformationResponse as u16).to_be_bytes();
        static DIAGNOSTIC_MESSAGE_BYTES: [u8; 2] =
            (PayloadType::DiagnosticMessage as u16).to_be_bytes();
        static DIAGNOSTIC_MESSAGE_ACK_BYTES: [u8; 2] =
            (PayloadType::DiagnosticMessageAck as u16).to_be_bytes();
        static DIAGNOSTIC_MESSAGE_NACK_BYTES: [u8; 2] =
            (PayloadType::DiagnosticMessageNack as u16).to_be_bytes();

        match self {
            PayloadType::GenericNack => &GENERIC_NACK_BYTES,
            PayloadType::VehicleIdentificationRequest => &VEHICLE_IDENTIFICATION_REQUEST_BYTES,
            PayloadType::VehicleIdentificationRequestEid => {
                &VEHICLE_IDENTIFICATION_REQUEST_EID_BYTES
            }
            PayloadType::VehicleIdentificationRequestVin => {
                &VEHICLE_IDENTIFICATION_REQUEST_VIN_BYTES
            }
            PayloadType::VehicleAnnouncementMessage => &VEHICLE_ANNOUNCEMENT_MESSAGE_BYTES,
            PayloadType::RoutingActivationRequest => &ROUTING_ACTIVATION_REQUEST_BYTES,
            PayloadType::RoutingActivationResponse => &ROUTING_ACTIVATION_RESPONSE_BYTES,
            PayloadType::AliveCheckRequest => &ALIVE_CHECK_REQUEST_BYTES,
            PayloadType::AliveCheckResponse => &ALIVE_CHECK_RESPONSE_BYTES,
            PayloadType::EntityStatusRequest => &ENTITY_STATUS_REQUEST_BYTES,
            PayloadType::EntityStatusResponse => &ENTITY_STATUS_RESPONSE_BYTES,
            PayloadType::PowerInformationRequest => &POWER_INFORMATION_REQUEST_BYTES,
            PayloadType::PowerInformationResponse => &POWER_INFORMATION_RESPONSE_BYTES,
            PayloadType::DiagnosticMessage => &DIAGNOSTIC_MESSAGE_BYTES,
            PayloadType::DiagnosticMessageAck => &DIAGNOSTIC_MESSAGE_ACK_BYTES,
            PayloadType::DiagnosticMessageNack => &DIAGNOSTIC_MESSAGE_NACK_BYTES,
        }
    }
}

fn validate_protocol(proto: u8, inv_proto: u8) -> Option<()> {
    if !proto == inv_proto {
        Some(())
    } else {
        None
    }
}

impl<const N: usize> Decoder<N> for HeaderCodec {
    type Item = DoipHeader;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN {
            return Err(DecodeError::TooShort);
        }

        let protocol_version_bytes = src[DOIP_VERSION_OFFSET];
        let protocol_version = ProtocolVersion::from_bytes(&[protocol_version_bytes])
            .ok_or(DecodeError::InvalidProtocolVersion)?;

        let inverse_protocol_version = src[DOIP_INV_VERSION_OFFSET];

        validate_protocol(protocol_version_bytes, inverse_protocol_version)
            .ok_or(DecodeError::FailedProtocolValidation)?;

        let payload_type_bytes = &src[DOIP_TYPE_OFFSET..(DOIP_TYPE_OFFSET + DOIP_TYPE_LEN)];
        let payload_type =
            PayloadType::from_bytes(payload_type_bytes).ok_or(DecodeError::InvalidPayloadType)?;

        let payload_length_bytes = &src[DOIP_LENGTH_OFFSET..(DOIP_LENGTH_OFFSET + DOIP_LENGTH_LEN)];
        let payload_length = u32::from_be_bytes(
            payload_length_bytes
                .try_into()
                .expect("Slice is always the correct length"),
        );

        let item = DoipHeader {
            protocol_version,
            inverse_protocol_version,
            payload_type,
            payload_length,
        };

        Ok(Some(item))
    }
}

impl FromBytes for ProtocolVersion {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let val = *bytes.first()?; // Use `get()` directly for safety

        match val {
            v if v == ProtocolVersion::ReservedVer as u8 => Some(ProtocolVersion::ReservedVer),
            v if v == ProtocolVersion::Iso13400_2010 as u8 => Some(ProtocolVersion::Iso13400_2010),
            v if v == ProtocolVersion::Iso13400_2012 as u8 => Some(ProtocolVersion::Iso13400_2012),
            v if v == ProtocolVersion::Iso13400_2019 as u8 => Some(ProtocolVersion::Iso13400_2019),
            v if v == ProtocolVersion::Iso13400_2019Amd1 as u8 => {
                Some(ProtocolVersion::Iso13400_2019Amd1)
            }
            v if v == ProtocolVersion::DefaultValue as u8 => Some(ProtocolVersion::DefaultValue),
            _ => None,
        }
    }
}

impl FromBytes for PayloadType {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let val = bytes.get(0..DOIP_TYPE_LEN)?;

        match val {
            v if v == (PayloadType::GenericNack as u16).to_be_bytes() => {
                Some(PayloadType::GenericNack)
            }
            v if v == (PayloadType::VehicleIdentificationRequest as u16).to_be_bytes() => {
                Some(PayloadType::VehicleIdentificationRequest)
            }
            v if v == (PayloadType::VehicleIdentificationRequestEid as u16).to_be_bytes() => {
                Some(PayloadType::VehicleIdentificationRequestEid)
            }
            v if v == (PayloadType::VehicleIdentificationRequestVin as u16).to_be_bytes() => {
                Some(PayloadType::VehicleIdentificationRequestVin)
            }
            v if v == (PayloadType::VehicleAnnouncementMessage as u16).to_be_bytes() => {
                Some(PayloadType::VehicleAnnouncementMessage)
            }
            v if v == (PayloadType::RoutingActivationRequest as u16).to_be_bytes() => {
                Some(PayloadType::RoutingActivationRequest)
            }
            v if v == (PayloadType::RoutingActivationResponse as u16).to_be_bytes() => {
                Some(PayloadType::RoutingActivationResponse)
            }
            v if v == (PayloadType::AliveCheckRequest as u16).to_be_bytes() => {
                Some(PayloadType::AliveCheckRequest)
            }
            v if v == (PayloadType::AliveCheckResponse as u16).to_be_bytes() => {
                Some(PayloadType::AliveCheckResponse)
            }
            v if v == (PayloadType::EntityStatusRequest as u16).to_be_bytes() => {
                Some(PayloadType::EntityStatusRequest)
            }
            v if v == (PayloadType::EntityStatusResponse as u16).to_be_bytes() => {
                Some(PayloadType::EntityStatusResponse)
            }
            v if v == (PayloadType::PowerInformationRequest as u16).to_be_bytes() => {
                Some(PayloadType::PowerInformationRequest)
            }
            v if v == (PayloadType::PowerInformationResponse as u16).to_be_bytes() => {
                Some(PayloadType::PowerInformationResponse)
            }
            v if v == (PayloadType::DiagnosticMessage as u16).to_be_bytes() => {
                Some(PayloadType::DiagnosticMessage)
            }
            v if v == (PayloadType::DiagnosticMessageAck as u16).to_be_bytes() => {
                Some(PayloadType::DiagnosticMessageAck)
            }
            v if v == (PayloadType::DiagnosticMessageNack as u16).to_be_bytes() => {
                Some(PayloadType::DiagnosticMessageNack)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{DoipPayload, GenericNack, NackCode},
        DoipMessage,
    };
    use heapless::Vec;

    use super::{validate_protocol, HeaderCodec};

    static SUCCESS_ROOT: DoipMessage<4095> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::GenericNack,
            payload_length: 1u32,
        },
        payload: DoipPayload::GenericNack(GenericNack {
            nack_code: NackCode::MessageTooLarge,
        }),
    };

    #[test]
    fn test_protocol_version_to_bytes() {
        let bytes = ProtocolVersion::ReservedVer.to_bytes();
        assert_eq!(bytes, &[0x00]);
        let bytes = ProtocolVersion::Iso13400_2010.to_bytes();
        assert_eq!(bytes, &[0x01]);
        let bytes = ProtocolVersion::Iso13400_2012.to_bytes();
        assert_eq!(bytes, &[0x02]);
        let bytes = ProtocolVersion::Iso13400_2019.to_bytes();
        assert_eq!(bytes, &[0x03]);
        let bytes = ProtocolVersion::Iso13400_2019Amd1.to_bytes();
        assert_eq!(bytes, &[0x04]);
        let bytes = ProtocolVersion::DefaultValue.to_bytes();
        assert_eq!(bytes, &[0xff]);
    }

    #[test]
    fn test_payload_type_to_bytes() {
        let bytes = PayloadType::GenericNack.to_bytes();
        assert_eq!(bytes, &[0x00, 0x00]);
        let bytes = PayloadType::VehicleIdentificationRequest.to_bytes();
        assert_eq!(bytes, &[0x00, 0x01]);
        let bytes = PayloadType::VehicleIdentificationRequestEid.to_bytes();
        assert_eq!(bytes, &[0x00, 0x02]);
        let bytes = PayloadType::VehicleIdentificationRequestVin.to_bytes();
        assert_eq!(bytes, &[0x00, 0x03]);
        let bytes = PayloadType::VehicleAnnouncementMessage.to_bytes();
        assert_eq!(bytes, &[0x00, 0x04]);
        let bytes = PayloadType::RoutingActivationRequest.to_bytes();
        assert_eq!(bytes, &[0x00, 0x05]);
        let bytes = PayloadType::RoutingActivationResponse.to_bytes();
        assert_eq!(bytes, &[0x00, 0x06]);
        let bytes = PayloadType::AliveCheckRequest.to_bytes();
        assert_eq!(bytes, &[0x00, 0x07]);
        let bytes = PayloadType::AliveCheckResponse.to_bytes();
        assert_eq!(bytes, &[0x00, 0x08]);
        let bytes = PayloadType::EntityStatusRequest.to_bytes();
        assert_eq!(bytes, &[0x40, 0x01]);
        let bytes = PayloadType::EntityStatusResponse.to_bytes();
        assert_eq!(bytes, &[0x40, 0x02]);
        let bytes = PayloadType::PowerInformationRequest.to_bytes();
        assert_eq!(bytes, &[0x40, 0x03]);
        let bytes = PayloadType::PowerInformationResponse.to_bytes();
        assert_eq!(bytes, &[0x40, 0x04]);
        let bytes = PayloadType::DiagnosticMessage.to_bytes();
        assert_eq!(bytes, &[0x80, 0x01]);
        let bytes = PayloadType::DiagnosticMessageAck.to_bytes();
        assert_eq!(bytes, &[0x80, 0x02]);
        let bytes = PayloadType::DiagnosticMessageNack.to_bytes();
        assert_eq!(bytes, &[0x80, 0x03]);
    }

    #[test]
    fn test_protocol_validation() {
        for a in u8::MIN..=u8::MAX {
            let proto = ProtocolVersion::ReservedVer.to_bytes()[0];
            let validate = validate_protocol(proto, a);
            match a {
                0xff => assert!(validate.is_some(), "Expected validation to succeed"),
                _ => assert!(validate.is_none(), "Expected validation to fail"),
            };
        }
        for a in u8::MIN..=u8::MAX {
            let proto = ProtocolVersion::Iso13400_2010.to_bytes()[0];
            let validate = validate_protocol(proto, a);
            match a {
                0xfe => assert!(validate.is_some(), "Expected validation to succeed"),
                _ => assert!(validate.is_none(), "Expected validation to fail"),
            };
        }
        for a in u8::MIN..=u8::MAX {
            let proto = ProtocolVersion::Iso13400_2012.to_bytes()[0];
            let validate = validate_protocol(proto, a);
            match a {
                0xfd => assert!(validate.is_some(), "Expected validation to succeed"),
                _ => assert!(validate.is_none(), "Expected validation to fail"),
            };
        }
        for a in u8::MIN..=u8::MAX {
            let proto = ProtocolVersion::Iso13400_2019.to_bytes()[0];
            let validate = validate_protocol(proto, a);
            match a {
                0xfc => assert!(validate.is_some(), "Expected validation to succeed"),
                _ => assert!(validate.is_none(), "Expected validation to fail"),
            };
        }
        for a in u8::MIN..=u8::MAX {
            let proto = ProtocolVersion::Iso13400_2019Amd1.to_bytes()[0];
            let validate = validate_protocol(proto, a);
            match a {
                0xfb => assert!(validate.is_some(), "Expected validation to succeed"),
                _ => assert!(validate.is_none(), "Expected validation to fail"),
            };
        }
        for a in u8::MIN..=u8::MAX {
            let proto = ProtocolVersion::DefaultValue.to_bytes()[0];
            let validate = validate_protocol(proto, a);
            match a {
                0x00 => assert!(validate.is_some(), "Expected validation to succeed"),
                _ => assert!(validate.is_none(), "Expected validation to fail"),
            };
        }
    }

    #[test]
    fn test_protocol_version_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = ProtocolVersion::from_bytes(bytes);

            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ProtocolVersion::ReservedVer)
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ProtocolVersion::Iso13400_2010)
                }
                0x02 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ProtocolVersion::Iso13400_2012)
                }
                0x03 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ProtocolVersion::Iso13400_2019)
                }
                0x04 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ProtocolVersion::Iso13400_2019Amd1)
                }
                0xff => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ProtocolVersion::DefaultValue)
                }
                _ => {
                    assert!(proto.is_none())
                }
            }
        }
    }

    #[test]
    fn test_payload_type_from_bytes() {
        for v in u16::MIN..=u16::MAX {
            let [a, b] = v.to_be_bytes();
            let bytes = &[a, b];
            let proto = PayloadType::from_bytes(bytes);

            match [a, b] {
                [0x00, 0x00] => {
                    assert!(proto.is_some(), "{:?} | {:?}", proto, [a, b]);
                    assert_eq!(proto.unwrap(), PayloadType::GenericNack)
                }
                [0x00, 0x01] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::VehicleIdentificationRequest)
                }
                [0x00, 0x02] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::VehicleIdentificationRequestEid)
                }
                [0x00, 0x03] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::VehicleIdentificationRequestVin)
                }
                [0x00, 0x04] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::VehicleAnnouncementMessage)
                }
                [0x00, 0x05] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::RoutingActivationRequest)
                }
                [0x00, 0x06] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::RoutingActivationResponse)
                }
                [0x00, 0x07] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::AliveCheckRequest)
                }
                [0x00, 0x08] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::AliveCheckResponse)
                }
                [0x40, 0x01] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::EntityStatusRequest)
                }
                [0x40, 0x02] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::EntityStatusResponse)
                }
                [0x40, 0x03] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::PowerInformationRequest)
                }
                [0x40, 0x04] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::PowerInformationResponse)
                }
                [0x80, 0x01] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::DiagnosticMessage)
                }
                [0x80, 0x02] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::DiagnosticMessageAck)
                }
                [0x80, 0x03] => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PayloadType::DiagnosticMessageNack)
                }
                _ => {
                    assert!(proto.is_none())
                }
            }
        }
    }

    #[test]
    fn test_encode_header_success() {
        let mut codec = HeaderCodec {};
        let mut dst = Vec::<u8, 4095>::new();
        let mut src = Vec::<u8, 4095>::new();

        let bytes = &[0x02, 0x0fd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        dst.extend_from_slice(bytes).unwrap();
        let item = codec.decode(&mut dst);

        let _ = codec.encode(item.unwrap().unwrap(), &mut src);

        assert_eq!(*src, *bytes)
    }

    #[test]
    fn test_encode_header_failed_protocol_validation() {
        let mut codec = HeaderCodec {};
        let mut src = Vec::<u8, 4095>::new();

        let res = codec.encode(
            DoipHeader {
                protocol_version: ProtocolVersion::Iso13400_2012,
                inverse_protocol_version: 0xff,
                payload_type: PayloadType::GenericNack,
                payload_length: 0u32,
            },
            &mut src,
        );

        assert!(res.is_err());
    }

    #[test]
    fn test_decode_header_success() {
        let mut codec = HeaderCodec {};
        let mut dst = Vec::<u8, 4095>::new();

        let _ = codec.encode(SUCCESS_ROOT.header.clone(), &mut dst);
        let msg = codec.decode(&mut dst);

        assert!(msg.is_ok());
        let opt = msg.unwrap();

        assert!(opt.is_some());
        let res = opt.unwrap();

        assert_eq!(res, SUCCESS_ROOT.header)
    }

    #[test]
    fn test_decode_header_failed_protocol_validation() {
        let mut codec = HeaderCodec {};
        let mut dst = Vec::<u8, 4095>::new();

        dst.extend_from_slice(&[0x02, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00])
            .unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_header_too_short() {
        let mut codec = HeaderCodec {};
        let mut dst = Vec::<u8, 4095>::new();

        dst.extend_from_slice(&[0x02, 0xff, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_header_invalid_protocol() {
        let mut codec = HeaderCodec {};
        let mut dst = Vec::<u8, 4095>::new();

        dst.extend_from_slice(&[0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_header_failed_protocol_check() {
        let mut codec = HeaderCodec {};
        let mut dst = Vec::<u8, 4095>::new();

        dst.extend_from_slice(&[0x02, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_header_invalid_payload_type() {
        let mut codec = HeaderCodec {};
        let mut dst = Vec::<u8, 4095>::new();

        dst.extend_from_slice(&[0x02, 0xfd, 0x90, 0x42, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }
}
