pub mod bytebuffer;
mod cursor;
mod error;
mod integer_compression;

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

#[cfg(feature = "cpp")]
#[cxx::bridge(namespace = "FastPForLib")]
mod ffi {
    unsafe extern "C++" {
        include!("fastpfor_bridge.h");

        type CODECFactory;
        type IntegerCODEC;

        fn new_codec_factory() -> UniquePtr<CODECFactory>;
        fn codec_factory_get_from_name(
            factory: &CODECFactory,
            name: &str,
        ) -> SharedPtr<IntegerCODEC>;
    }
}

#[cfg(all(test, feature = "cpp"))]
mod tests {
    use super::*;

    #[test]
    fn test_instantiation() {
        let factory = ffi::new_codec_factory();
        let codec = ffi::codec_factory_get_from_name(&factory, "FastPFor");
        assert!(!codec.is_null());
    }
}
