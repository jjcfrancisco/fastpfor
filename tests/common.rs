use fastpfor::{Codec, Composition, FastPFOR, VariableByte, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};

pub fn get_codecs() -> Vec<Codec> {
    let variablebyte = VariableByte;

    vec![
        Codec::VariableByte(variablebyte),
        Codec::FastPFor(FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_256)),
        Codec::Composition(Box::new(Composition::new(
            Codec::FastPFor(FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_256)),
            Codec::VariableByte(VariableByte),
        ))),
    ]
}
