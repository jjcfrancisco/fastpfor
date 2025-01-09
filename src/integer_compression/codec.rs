use crate::{FastPFOR, VariableByte};

pub enum Codec {
    FastPFor(FastPFOR),
    VariableByte(VariableByte),
}

impl From<FastPFOR> for Codec {
    fn from(fastpfor: FastPFOR) -> Self {
        Codec::FastPFor(fastpfor)
    }
}

impl From<VariableByte> for Codec {
    fn from(vb: VariableByte) -> Self {
        Codec::VariableByte(vb)
    }
}

