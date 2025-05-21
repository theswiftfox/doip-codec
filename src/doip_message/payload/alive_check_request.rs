use doip_definitions::{
    definitions::DOIP_HEADER_LEN,
    payload::{AliveCheckRequest, DoipPayload},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct AliveCheckRequestCodec;

impl<const N: usize> Encoder<AliveCheckRequest, N> for AliveCheckRequestCodec {
    type Error = EncodeError;

    fn to_bytes(
        &mut self,
        item: AliveCheckRequest,
        _dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let AliveCheckRequest {} = item;

        Ok(())
    }
}

impl<const N: usize> Decoder<N> for AliveCheckRequestCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn from_bytes(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN {
            return Err(DecodeError::TooShort);
        }

        Ok(Some(DoipPayload::AliveCheckRequest(AliveCheckRequest {})))
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        message::DoipMessage,
        payload::{AliveCheckRequest, DoipPayload},
    };
    use heapless::Vec;

    use crate::{
        doip_message::payload::alive_check_request::AliveCheckRequestCodec, Decoder, DoipCodec,
        Encoder,
    };

    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::AliveCheckRequest,
            payload_length: 0u32,
        },
        payload: DoipPayload::AliveCheckRequest(AliveCheckRequest {}),
    };

    #[test]
    fn test_encode_alive_check_request_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.to_bytes(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(*dst, [0x02, 0xfd, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_decode_alive_check_request_success() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let _ = codec.to_bytes(SUCCESS_ROOT.clone(), &mut dst);
        let msg = codec.from_bytes(&mut dst);

        assert!(msg.is_ok());
        let opt = msg.unwrap();

        assert!(opt.is_some());
        let res = opt.unwrap();

        assert_eq!(res, SUCCESS_ROOT);
    }

    #[test]
    fn test_decode_alive_check_request_too_short() {
        let mut codec = AliveCheckRequestCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x07, 0x00, 0x00, 0x00];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.from_bytes(&mut dst);

        assert!(msg.is_err());
    }
}
