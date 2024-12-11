use crate::{doip::message::message::DoipMessage, error::DecodeError, DoipCodec};
use tokio_util::bytes::{Buf, BytesMut};

pub const MAX: usize = 8 * 1024 * 1024;

impl tokio_util::codec::Decoder for DoipCodec {
    type Item = DoipMessage;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        };
        // Check that the length is not too large to avoid a denial of
        // service attack where the server runs out of memory.
        if src.len() > MAX {
            return Err(DecodeError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", src.len()),
            )));
        }

        // Use advance to modify src such that it no longer contains
        // this frame.
        let data = src[..src.len()].to_vec();
        // src.advance(src.len());

        match DoipMessage::parse_from_bytes(data) {
            Ok(msg) => {
                src.advance(msg.header.payload_length as usize + 8);
                Ok(Some(msg))
            }
            Err(err) => Err(DecodeError::ParseError(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip::{
            header::{header::DoipHeader, payload::payload::PayloadType, version::DoipVersion},
            message::message::DoipMessage,
        },
        DoipCodec,
    };
    use tokio_util::{bytes::BytesMut, codec::Decoder};

    #[test]
    fn test_decode_single_message() {
        let mut decoder = DoipCodec {};
        let mut bytes = BytesMut::new();

        let src = [0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        bytes.extend_from_slice(&src);

        let msg = decoder.decode(&mut bytes);
        assert!(msg.is_ok(), "Expected to receive a result.");

        assert!(
            msg.as_ref().unwrap().is_some(),
            "Expected to receive a defined message."
        );

        let res = msg.unwrap().unwrap();
        let payload_type = (&*res.payload).payload_type();
        let payload_len = (&*res.payload).to_bytes().len();

        assert_eq!(
            res.header,
            DoipHeader {
                protocol_version: DoipVersion::Iso13400_2012,
                inverse_protocol_version: 0xfd,
                payload_type: PayloadType::VehicleIdentificationRequest,
                payload_length: 0
            },
            "Unexpected message: {:?}",
            res
        );

        assert_eq!(
            payload_type,
            PayloadType::VehicleIdentificationRequest,
            "Unexpected payload: {:?}",
            payload_type
        );

        assert_eq!(payload_len, 0, "Unexpected payload: {:?}", payload_len);
    }

    #[test]
    fn test_decode_single_message_short() {
        let mut decoder = DoipCodec {};
        let mut bytes = BytesMut::new();

        let src = [0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00];
        bytes.extend_from_slice(&src);

        let msg = decoder.decode(&mut bytes);

        assert!(
            msg.is_err(),
            "Expected to receive an Err(ParseError::IndexFailure)."
        );
    }

    #[test]
    fn test_decode_many_message() {
        let mut decoder = DoipCodec {};
        let mut bytes = BytesMut::new();
        let number_of_messages: usize = 30;

        let src = [0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00].repeat(number_of_messages);
        bytes.extend_from_slice(&src);

        let mut messages: Vec<DoipMessage> = vec![];

        while bytes.len() >= 8 {
            let msg = decoder.decode(&mut bytes).unwrap();

            let _ = match msg {
                Some(msg) => messages.push(msg),
                None => continue,
            };
        }

        assert_eq!(
            messages.len(),
            number_of_messages,
            "Unexpected messages: {:?}",
            messages
        );
    }
}
