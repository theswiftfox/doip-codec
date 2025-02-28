use doip_definitions::payload::DoipMessage;
use tokio_util::bytes::BytesMut;

use crate::{error::DecodeError, Decoder, DoipCodec};

pub const MAX: usize = 8 * 1024 * 1024;

impl Decoder for DoipCodec {
    type Item = DoipMessage<'static>;
    type Error = DecodeError;

    fn decode<'a>(&'a mut self, src: &'a mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
