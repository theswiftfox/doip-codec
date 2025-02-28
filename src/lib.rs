#![no_std]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//! # Diagnostics over Internet Protocol Codec Crate
//!
//! The purpose of this crate is to provide an easy way to encode and decode
//! DoIP Messages defined in the `doip-definitions` crate.
//!
//! ## Example Usage
//! ```no_run
//! use futures::{SinkExt, StreamExt};
//! use tokio::net::TcpStream;
//! use tokio_util::codec::Framed;
//! use doip_definitions::{
//!     header::DoipVersion,
//!     message::{DoipMessage, VehicleIdentificationRequest},
//! };
//! use doip_codec::DoipCodec;
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
mod doip_message;
mod encoder;
mod error;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::{bytes::BytesMut, codec::Framed};

pub use crate::error::*;

/// A simple Decoder and Encoder implementation for Diagnostics over Internet
/// Protocol.
///
/// Can be used independently via `encode` and `decode` methods, however is best
/// utilised during.
#[derive(Debug)]
pub struct DoipCodec;

pub trait Decoder {
    type Item;
    type Error: From<DecodeError>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>;

    fn framed<T: AsyncRead + AsyncWrite + Sized>(self, io: T) -> Framed<T, Self>
    where
        Self: Sized,
    {
        Framed::new(io, self)
    }
}

pub trait Encoder<Item> {
    type Error: From<EncodeError>;

    fn encode(&mut self, item: Item, dst: &mut BytesMut) -> Result<(), Self::Error>;
}
