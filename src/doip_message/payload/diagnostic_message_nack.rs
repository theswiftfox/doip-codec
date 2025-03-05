use doip_definitions::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_COMMON_TARGET_OFFSET,
        DOIP_DIAG_MESSAGE_NACK_CODE_LEN, DOIP_DIAG_MESSAGE_NACK_CODE_OFFSET, DOIP_HEADER_LEN,
    },
    payload::{DiagnosticMessageNack, DiagnosticNackCode, DoipPayload},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};

#[derive(Debug)]
pub struct DiagnosticMessageNackCodec;

impl<const N: usize> Encoder<DiagnosticMessageNack, N> for DiagnosticMessageNackCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: DiagnosticMessageNack,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let DiagnosticMessageNack {
            source_address,
            target_address,
            nack_code,
        } = item;

        dst.extend_from_slice(&source_address).map_err(|()| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&target_address).map_err(|()| EncodeError::BufferTooSmall)?;

        let nack_code_bytes = nack_code.to_bytes();
        dst.extend_from_slice(nack_code_bytes).map_err(|()| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl ToBytes for DiagnosticNackCode {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            DiagnosticNackCode::ReservedByIso13400_00 => {
                &[DiagnosticNackCode::ReservedByIso13400_00 as u8]
            }
            DiagnosticNackCode::ReservedByIso13400_01 => {
                &[DiagnosticNackCode::ReservedByIso13400_01 as u8]
            }
            DiagnosticNackCode::InvalidSourceAddress => {
                &[DiagnosticNackCode::InvalidSourceAddress as u8]
            }
            DiagnosticNackCode::UnknownTargetAddress => {
                &[DiagnosticNackCode::UnknownTargetAddress as u8]
            }
            DiagnosticNackCode::DiagnosticMessageTooLarge => {
                &[DiagnosticNackCode::DiagnosticMessageTooLarge as u8]
            }
            DiagnosticNackCode::OutOfMemory => &[DiagnosticNackCode::OutOfMemory as u8],
            DiagnosticNackCode::TargetUnreachable => &[DiagnosticNackCode::TargetUnreachable as u8],
            DiagnosticNackCode::UnknownNetwork => &[DiagnosticNackCode::UnknownNetwork as u8],
            DiagnosticNackCode::TransportProtocolError => {
                &[DiagnosticNackCode::TransportProtocolError as u8]
            }
        }
    }
}

impl<const N: usize> Decoder<N> for DiagnosticMessageNackCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len()
            < DOIP_HEADER_LEN
                + DOIP_DIAG_COMMON_SOURCE_LEN
                + DOIP_DIAG_COMMON_TARGET_LEN
                + DOIP_DIAG_MESSAGE_NACK_CODE_LEN
        {
            return Err(DecodeError::TooShort);
        };

        let source_address = src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_DIAG_COMMON_SOURCE_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let target_address = src[DOIP_DIAG_COMMON_TARGET_OFFSET
            ..DOIP_DIAG_COMMON_TARGET_OFFSET + DOIP_DIAG_COMMON_TARGET_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let nack_code_bytes = &src[DOIP_DIAG_MESSAGE_NACK_CODE_OFFSET..=DOIP_DIAG_MESSAGE_NACK_CODE_OFFSET];
        let nack_code = DiagnosticNackCode::from_bytes(nack_code_bytes)
            .ok_or(DecodeError::InvalidDiagnosticNackCode)?;

        let item = DiagnosticMessageNack {
            source_address,
            target_address,
            nack_code,
        };

        Ok(Some(DoipPayload::DiagnosticMessageNack(item)))
    }
}

impl FromBytes for DiagnosticNackCode {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        match val {
            v if v == DiagnosticNackCode::ReservedByIso13400_00 as u8 => {
                Some(DiagnosticNackCode::ReservedByIso13400_00)
            }
            v if v == DiagnosticNackCode::ReservedByIso13400_01 as u8 => {
                Some(DiagnosticNackCode::ReservedByIso13400_01)
            }
            v if v == DiagnosticNackCode::InvalidSourceAddress as u8 => {
                Some(DiagnosticNackCode::InvalidSourceAddress)
            }
            v if v == DiagnosticNackCode::UnknownTargetAddress as u8 => {
                Some(DiagnosticNackCode::UnknownTargetAddress)
            }
            v if v == DiagnosticNackCode::OutOfMemory as u8 => {
                Some(DiagnosticNackCode::OutOfMemory)
            }
            v if v == DiagnosticNackCode::DiagnosticMessageTooLarge as u8 => {
                Some(DiagnosticNackCode::DiagnosticMessageTooLarge)
            }
            v if v == DiagnosticNackCode::TargetUnreachable as u8 => {
                Some(DiagnosticNackCode::TargetUnreachable)
            }
            v if v == DiagnosticNackCode::UnknownNetwork as u8 => {
                Some(DiagnosticNackCode::UnknownNetwork)
            }
            v if v == DiagnosticNackCode::TransportProtocolError as u8 => {
                Some(DiagnosticNackCode::TransportProtocolError)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{DiagnosticMessageNack, DiagnosticNackCode, DoipPayload},
        DoipMessage,
    };
    use heapless::Vec;

    use crate::{Decoder, DoipCodec, Encoder, FromBytes, ToBytes};

    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::DiagnosticMessageNack,
            payload_length: 5u32,
        },
        payload: DoipPayload::DiagnosticMessageNack(DiagnosticMessageNack {
            source_address: [0x00, 0x00],
            target_address: [0x00, 0x00],
            nack_code: DiagnosticNackCode::DiagnosticMessageTooLarge,
        }),
    };

    #[test]
    fn test_nack_code_to_bytes() {
        let bytes = DiagnosticNackCode::ReservedByIso13400_00.to_bytes();
        assert_eq!(bytes, [0x00]);
        let bytes = DiagnosticNackCode::ReservedByIso13400_01.to_bytes();
        assert_eq!(bytes, [0x01]);
        let bytes = DiagnosticNackCode::InvalidSourceAddress.to_bytes();
        assert_eq!(bytes, [0x02]);
        let bytes = DiagnosticNackCode::UnknownTargetAddress.to_bytes();
        assert_eq!(bytes, [0x03]);
        let bytes = DiagnosticNackCode::DiagnosticMessageTooLarge.to_bytes();
        assert_eq!(bytes, [0x04]);
        let bytes = DiagnosticNackCode::OutOfMemory.to_bytes();
        assert_eq!(bytes, [0x05]);
        let bytes = DiagnosticNackCode::TargetUnreachable.to_bytes();
        assert_eq!(bytes, [0x06]);
        let bytes = DiagnosticNackCode::UnknownNetwork.to_bytes();
        assert_eq!(bytes, [0x07]);
        let bytes = DiagnosticNackCode::TransportProtocolError.to_bytes();
        assert_eq!(bytes, [0x08]);
    }

    #[test]
    fn test_nack_code_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = DiagnosticNackCode::from_bytes(bytes);

            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticNackCode::ReservedByIso13400_00)
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticNackCode::ReservedByIso13400_01)
                }
                0x02 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticNackCode::InvalidSourceAddress)
                }
                0x03 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticNackCode::UnknownTargetAddress)
                }
                0x04 => {
                    assert!(proto.is_some());
                    assert_eq!(
                        proto.unwrap(),
                        DiagnosticNackCode::DiagnosticMessageTooLarge
                    )
                }
                0x05 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticNackCode::OutOfMemory)
                }
                0x06 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticNackCode::TargetUnreachable)
                }
                0x07 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticNackCode::UnknownNetwork)
                }
                0x08 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticNackCode::TransportProtocolError)
                }
                _ => assert!(proto.is_none()),
            }
        }
    }

    #[test]
    fn test_encode_diagnostic_message_ack_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [0x02, 0xfd, 0x80, 0x03, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x04]
        );
    }

    #[test]
    fn test_decode_diagnostic_message_nack_success() {
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
    fn test_decode_diagnostic_message_nack_invalid_diagnostic_nack_code() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x80, 0x03, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x42,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_diagnostic_message_nack_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x80, 0x03, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }
}
