use doip_definitions::message::DoipMessage;

use crate::{error::EncodeError, DoipCodec};

impl tokio_util::codec::Encoder<DoipMessage> for DoipCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: DoipMessage,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let msg = item.to_bytes();

        dst.reserve(msg.len());
        dst.extend_from_slice(&msg);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::DoipCodec;
    use doip_definitions::{
        header::DoipVersion,
        message::{DoipMessage, VehicleIdentificationRequest},
    };
    use tokio_util::{bytes::BytesMut, codec::Encoder};

    #[test]
    fn test_encode_message() {
        let mut encoder = DoipCodec {};
        let mut dst = BytesMut::new();

        let item = DoipMessage::new(
            DoipVersion::Iso13400_2012,
            Box::new(VehicleIdentificationRequest {}),
        );

        let bytes = encoder.encode(item, &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");

        assert_eq!(*dst, [0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_encode_many_messages() {
        let mut encoder = DoipCodec {};
        let mut dst = BytesMut::new();

        let item_1 = DoipMessage::new(
            DoipVersion::Iso13400_2012,
            Box::new(VehicleIdentificationRequest {}),
        );

        let item_2 = DoipMessage::new(
            DoipVersion::Iso13400_2012,
            Box::new(VehicleIdentificationRequest {}),
        );

        let bytes_1 = encoder.encode(item_1, &mut dst);
        let bytes_2 = encoder.encode(item_2, &mut dst);

        assert!(bytes_1.is_ok(), "Expected bytes_1 to be ok.");
        assert!(bytes_2.is_ok(), "Expected bytes_2 to be ok.");

        assert_eq!(
            *dst,
            [
                0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x02, 0xfd, 0x00, 0x01, 0x00, 0x00,
                0x00, 0x00
            ]
        );
    }
}
