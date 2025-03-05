#![no_std]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//! # Diagnostics over Internet Protocol Codec Crate
//!
//! The purpose of this crate is to provide an easy way to encode and decode
//! `DoIP` Messages defined in the `doip-definitions` crate.
//!
//!

mod decoder;
mod doip_message;
mod encoder;
mod error;

pub use crate::error::*;
use heapless::Vec;

/// A simple Decoder and Encoder implementation for Diagnostics over Internet
/// Protocol.
///
/// Can be used independently via `encode` and `decode` methods, however is best
/// utilised during.
#[derive(Debug)]
pub struct DoipCodec<const N: usize> {}

/// Decoder trait to decode inbound messages from a source and produce human-readable and programmable
/// output. Similar but adapted from the `tokio_utils` Decoder to be used within a `no_std` environment.
pub trait Decoder<const N: usize> {
    /// The type of decoded frames
    type Item;
    /// The type of unrecoverable frame decoding errors.
    ///
    /// If an individual message is ill-formed but can be ignored without interfering with the
    /// processing of future messages, it may be more useful to report the failure as an Item.
    type Error: From<DecodeError>;

    /// Attempts to decode a frame from the provided buffer of bytes.
    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error>;
}

/// Encoder trait to encode runtime or compile time messages for diagnsotic applications into streamable
/// bytes. Similar but adapted from the `tokio_utils` Encoder to be used within a `no_std` environment.
pub trait Encoder<Item, const N: usize> {
    /// The type of encoding errors.
    type Error: From<EncodeError>;

    /// Encodes a frame into the buffer provided.
    fn encode(&mut self, item: Item, dst: &mut Vec<u8, N>) -> Result<(), Self::Error>;
}

trait ToBytes {
    fn to_bytes(self) -> &'static [u8];
}

trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
}

// Panic handler for `no_std` environments (only when `std` is NOT enabled)
#[cfg(all(not(feature = "std"), not(test)))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
