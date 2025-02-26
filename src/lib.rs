pub mod bytebuffer;
mod cursor;
mod error;
mod integer_compression;

// FIXME: need decide on the external API. Some ideas:
//  - offer two sets of similar APIs - rust and cpp ffi
//  - it will be possible to enable/disable each with a feature flag
//  - introduce a new feature-agnostic API that will forward to either
//  - if both are enabled, forward to the more stable (ffi probably)
#[cfg(feature = "cpp")]
mod cpp;

#[cfg(feature = "cpp")]
pub use cpp::{Codec as FfiCodec, CodecFactory as FfiCodecFactory};
pub use error::{FastPForError, FastPForResult};
pub use integer_compression::bitpacking::{fast_pack, fast_unpack};
pub use integer_compression::codec::Codec;
pub use integer_compression::composition::Composition;
pub use integer_compression::differential::Delta;
pub use integer_compression::fastpfor::{
    FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE,
};
pub use integer_compression::integer_codec::Integer;
pub use integer_compression::just_copy::JustCopy;
pub use integer_compression::skippable_codec::Skippable;
pub use integer_compression::variable_byte::VariableByte;

#[derive(Debug, PartialEq)]
pub enum Output {
    Byte(Vec<u8>),
    I32(Vec<i32>),
}
