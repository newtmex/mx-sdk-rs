use elrond_wasm::Vec;
use elrond_wasm::elrond_codec::*;

/// Copied from elrond-wasm serialization tests. 
pub struct SerExample1 {
    pub int: u16,
    pub seq: Vec<u8>,
    pub another_byte: u8,
}

impl NestedEncode for SerExample1 {
    fn dep_encode_to<O: NestedOutputBuffer>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.int.dep_encode_to(dest)?;
        self.seq.dep_encode_to(dest)?;
        self.another_byte.dep_encode_to(dest)?;
        Ok(())
    }
}

impl NestedDecode for SerExample1 {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(SerExample1{
            int: u16::dep_decode(input)?,
            seq: Vec::<u8>::dep_decode(input)?,
            another_byte: u8::dep_decode(input)?,
        })
    }
}

impl TopDecode for SerExample1 {
    fn top_decode<I: TopDecodeInput>(mut input: I) -> Result<Self, DecodeError> {
        dep_decode_from_byte_slice(input.get_slice_u8())
    }
}
