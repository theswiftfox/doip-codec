use doip_definitions::message::DoipMessage;

use crate::DoipCodec;
use crate::{Error, Result};

// region:      --- no_std
#[cfg(not(feature = "std"))]
pub trait Encoder<Item, const N: usize> {
    /// The type of decoded frames
    type Item;

    /// The type of unrecoverable frame decoding errors.
    ///
    /// If an individual message is ill-formed but can be ignored without interfering with the
    /// processing of future messages, sit may be more useful to report the failure as an Item.
    type Error: From<Error>;

    /// Attempts to decode a frame from the provided buffer of bytes.
    fn encode(&mut self, item: Item, dst: &mut heapless::Vec<u8, N>) -> Result<()>;
}

#[cfg(not(feature = "std"))]
impl<const N: usize> Encoder<DoipMessage<N>, N> for DoipCodec<N> {
    type Item = DoipMessage<N>;
    type Error = Error;

    fn encode(&mut self, item: DoipMessage<N>, dst: &mut heapless::Vec<u8, N>) -> Result<()> {
        let header: [u8; 8] = item.header.into();
        dst.extend_from_slice(&header)?;

        let payload: [u8; N] = item.payload.into();
        dst.extend_from_slice(&payload)?;

        Ok(())
    }
}

// endregion:   --- no_std

// region:      --- std

#[cfg(feature = "std")]
impl tokio_util::codec::Encoder<DoipMessage> for DoipCodec {
    type Error = Error;

    fn encode(&mut self, item: DoipMessage, dst: &mut tokio_util::bytes::BytesMut) -> Result<()> {
        let payload_len = item.header.payload_length as usize;
        let header: [u8; 8] = item.header.into();

        dst.reserve(8 + payload_len);
        dst.extend_from_slice(&header);

        let payload: Vec<u8> = item.payload.into();
        dst.extend_from_slice(&payload);

        Ok(())
    }
}

// endregion:   --- std
