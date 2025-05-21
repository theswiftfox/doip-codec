use doip_definitions::{
    definitions::{DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN, DOIP_HEADER_LEN},
    payload::{AliveCheckResponse, DoipPayload},
};

use crate::{DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct AliveCheckResponseCodec;

impl Encoder<AliveCheckResponse> for AliveCheckResponseCodec {
    type Error = EncodeError;

    fn to_bytes(&mut self, item: AliveCheckResponse, dst: &mut Vec<u8>) -> Result<(), Self::Error> {
        let AliveCheckResponse { source_address } = item;

        dst.extend_from_slice(&source_address);

        Ok(())
    }
}

impl Decoder for AliveCheckResponseCodec {
    type Item = DoipPayload;

    type Error = DecodeError;

    fn decode_from_bytes(&mut self, src: &mut Vec<u8>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN {
            return Err(DecodeError::TooShort);
        }

        let source_address = src
            [DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let item = AliveCheckResponse { source_address };

        Ok(Some(DoipPayload::AliveCheckResponse(item)))
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        message::DoipMessage,
        payload::{AliveCheckResponse, DoipPayload},
    };

    use crate::{Decoder, DoipCodec, Encoder};

    static SUCCESS_ROOT: DoipMessage = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::AliveCheckResponse,
            payload_length: 2u32,
        },
        payload: DoipPayload::AliveCheckResponse(AliveCheckResponse {
            source_address: [0x00, 0x00],
        }),
    };

    #[test]
    fn test_encode_alive_check_response_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let bytes = encoder.to_bytes(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00]
        );
    }

    #[test]
    fn test_decode_alive_check_response_success() {
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
    fn test_decode_alive_check_response_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x00];
        dst.extend_from_slice(bytes);
        let msg = codec.decode_from_bytes(&mut dst);

        assert!(msg.is_err());
    }
}
