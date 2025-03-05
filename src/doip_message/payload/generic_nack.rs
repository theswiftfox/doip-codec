use doip_definitions::{
    definitions::{DOIP_GENERIC_NACK_LEN, DOIP_HEADER_LEN},
    payload::{DoipPayload, GenericNack, NackCode},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};

#[derive(Debug)]
pub struct GenericNackCodec;

impl<const N: usize> Encoder<GenericNack, N> for GenericNackCodec {
    type Error = EncodeError;

    fn encode(&mut self, item: GenericNack, dst: &mut Vec<u8, N>) -> Result<(), Self::Error> {
        let GenericNack { nack_code } = item;

        let bytes = nack_code.to_bytes();

        dst.extend_from_slice(bytes).map_err(|()| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl ToBytes for NackCode {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            NackCode::IncorrectPatternFormat => &[NackCode::IncorrectPatternFormat as u8],
            NackCode::UnknownPayloadType => &[NackCode::UnknownPayloadType as u8],
            NackCode::MessageTooLarge => &[NackCode::MessageTooLarge as u8],
            NackCode::OutOfMemory => &[NackCode::OutOfMemory as u8],
            NackCode::InvalidPayloadLength => &[NackCode::InvalidPayloadLength as u8],
        }
    }
}

impl<const N: usize> Decoder<N> for GenericNackCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_GENERIC_NACK_LEN {
            return Err(DecodeError::TooShort);
        }

        let nack_code_bytes = &src[DOIP_HEADER_LEN..=DOIP_HEADER_LEN];

        let nack_code =
            NackCode::from_bytes(nack_code_bytes).ok_or(DecodeError::InvalidNackCode)?;

        let item = GenericNack { nack_code };

        Ok(Some(DoipPayload::GenericNack(item)))
    }
}

impl FromBytes for NackCode {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        match val {
            a if a == NackCode::IncorrectPatternFormat as u8 => {
                Some(NackCode::IncorrectPatternFormat)
            }
            b if b == NackCode::UnknownPayloadType as u8 => Some(NackCode::UnknownPayloadType),
            c if c == NackCode::OutOfMemory as u8 => Some(NackCode::OutOfMemory),
            d if d == NackCode::MessageTooLarge as u8 => Some(NackCode::MessageTooLarge),
            e if e == NackCode::InvalidPayloadLength as u8 => Some(NackCode::InvalidPayloadLength),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::payload::generic_nack::GenericNackCodec, DecodeError, Decoder, DoipCodec,
        Encoder, FromBytes, ToBytes,
    };
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{DoipPayload, GenericNack, NackCode},
        DoipMessage,
    };
    use heapless::Vec;
    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
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
    fn test_decode_generic_nack_success() {
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
    fn test_decode_generic_nack_invalid_payload() {
        let mut codec = GenericNackCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_generic_nack_invalid_nack() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xff];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_encode_single_message_generic_nack_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(*dst, [0x02, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02])
    }

    #[test]
    fn test_nack_code_to_bytes() {
        let bytes = NackCode::IncorrectPatternFormat.to_bytes();
        assert_eq!(bytes, &[0x00]);
        let bytes = NackCode::UnknownPayloadType.to_bytes();
        assert_eq!(bytes, &[0x01]);
        let bytes = NackCode::MessageTooLarge.to_bytes();
        assert_eq!(bytes, &[0x02]);
        let bytes = NackCode::OutOfMemory.to_bytes();
        assert_eq!(bytes, &[0x03]);
        let bytes = NackCode::InvalidPayloadLength.to_bytes();
        assert_eq!(bytes, &[0x04]);
    }

    #[test]
    fn test_nack_code_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = NackCode::from_bytes(bytes);

            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), NackCode::IncorrectPatternFormat)
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), NackCode::UnknownPayloadType)
                }
                0x02 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), NackCode::MessageTooLarge)
                }
                0x03 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), NackCode::OutOfMemory)
                }
                0x04 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), NackCode::InvalidPayloadLength)
                }
                _ => {
                    assert!(proto.is_none())
                }
            }
        }
    }
}
