use fastpfor::{Codec, VariableByte};

pub fn get_integer_codecs() -> Vec<Codec> {
    let variablebyte = VariableByte;
    vec![Codec::VariableByte(variablebyte)]
}
