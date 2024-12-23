mod decoder;
pub mod doip;
mod encoder;
pub mod error;

#[derive(Debug)]
pub struct DoipCodec;

#[cfg(test)]
mod tests {
    use tokio_util::codec::{FramedRead, FramedWrite};

    use crate::{
        doip::{
            header::{
                payload::vehicle_identification_request::VehicleIdentificationRequest,
                version::DoipVersion,
            },
            message::message::DoipMessage,
        },
        DoipCodec,
    };

    #[tokio::test]
    async fn test_framed_write() {
        use futures::sink::SinkExt;

        let buffer = Vec::new();

        let item_1 = DoipMessage::new(
            DoipVersion::Iso13400_2012,
            Box::new(VehicleIdentificationRequest {}),
        );

        let item_2 = DoipMessage::new(
            DoipVersion::Iso13400_2012,
            Box::new(VehicleIdentificationRequest {}),
        );
        let encoder = DoipCodec;

        let mut writer = FramedWrite::new(buffer, encoder);

        writer.send(item_1).await.unwrap();
        writer.send(item_2).await.unwrap();

        let buffer = writer.get_ref();

        assert_eq!(
            buffer.as_slice(),
            [
                0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x02, 0xfd, 0x00, 0x01, 0x00, 0x00,
                0x00, 0x00
            ]
        );
    }

    #[tokio::test]
    async fn test_framed_read() {
        use futures::StreamExt;

        let bytes = vec![
            0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x02, 0xfd, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00,
        ];
        let decoder = DoipCodec;

        let mut reader = FramedRead::new(bytes.as_slice(), decoder);

        let frame_1 = reader.next().await.unwrap().unwrap();
        let frame_2 = reader.next().await.unwrap().unwrap();

        assert!(reader.next().await.is_none());

        assert_eq!(
            frame_1.header,
            DoipMessage::new(
                DoipVersion::Iso13400_2012,
                Box::new(VehicleIdentificationRequest {}),
            )
            .header
        );

        assert_eq!(
            frame_2.header,
            DoipMessage::new(
                DoipVersion::Iso13400_2012,
                Box::new(VehicleIdentificationRequest {}),
            )
            .header
        );
    }
}
