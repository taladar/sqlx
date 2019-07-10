use super::super::{client::TextProtocol, encode::Encoder, serialize::Serialize, types::Capabilities};
use failure::Error;

pub struct ComQuit();

impl Serialize for ComQuit {
    fn serialize<'a, 'b>(
        &self,
        encoder: &'b mut Encoder<'a>,
        _server_capabilities: &Capabilities,
    ) -> Result<(), Error> {
        encoder.encode_int_1(TextProtocol::ComQuit.into());

        Ok(())
    }
}
