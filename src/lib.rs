#![warn(missing_docs)]

//! # Diagnostics over Internet Protocol Codec Crate
//!
//! The purpose of this crate is to provide an easy way to encode and decode
//! DoIP Messages defined in the `doip-definitions` crate.
//!
//! ## Example Usage
//! ```rust
//! use futures::{SinkExt, StreamExt};
//! use tokio::net::TcpStream;
//! use tokio_util::codec::Framed;
//! use doip_definitions::{
//!     header::DoipVersion,
//!     message::{DoipMessage, VehicleIdentificationRequest},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   // Connect to a DoIP server
//!   let stream = TcpStream::connect("127.0.0.1:13400").await?;
//!
//!   // Wrap the stream with the DoipCodec
//!   let mut framed = Framed::new(stream, DoipCodec);
//!
//!   // Send a DoIP message
//!   let request = DoipMessage::new(
//!       DoipVersion::Iso13400_2012,
//!       Box::new(VehicleIdentificationRequest {}),
//!   ); // Example payload
//!
//!   framed.send(request).await?;
//!
//!   // Receive a DoIP message
//!   if let Some(response) = framed.next().await {
//!       match response {
//!           Ok(msg) => println!("Received message: {:?}", msg),
//!           Err(e) => eprintln!("Failed to decode message: {}", e),
//!       }
//!   }
//!
//!   Ok(())
//! }
//! ```
//!

mod decoder;
mod encoder;
mod error;
pub use crate::error::*;

/// A simple Decoder and Encoder implementation for Diagnostics over Internet
/// Protocol.
///
/// Can be used independently via `encode` and `decode` methods, however is best
/// utilised during.
#[derive(Debug)]
pub struct DoipCodec;

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::DoipVersion,
        message::{DoipMessage, VehicleIdentificationRequest},
    };
    use tokio_util::codec::{FramedRead, FramedWrite};

    use crate::DoipCodec;

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
