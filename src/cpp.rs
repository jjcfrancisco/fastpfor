use cxx::{SharedPtr, UniquePtr};

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

        // Encode method: returns number of output values
        fn codec_encode(
            codec: &SharedPtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u32],
        ) -> Result<usize>;

        // Decode method: returns consumed input values, returns number of output values
        fn codec_decode(
            codec: &SharedPtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u32],
        ) -> Result<usize>;
    }
}

pub struct CodecFactory {
    factory: UniquePtr<ffi::CODECFactory>,
}

impl CodecFactory {
    pub fn new() -> Option<Self> {
        // TODO: is it ever possible for it to be null?
        let factory = ffi::new_codec_factory();
        if factory.is_null() {
            None
        } else {
            Some(Self { factory })
        }
    }

    pub fn get_codec(&self, name: &str) -> Option<Codec> {
        let codec = ffi::codec_factory_get_from_name(&self.factory, name);
        if codec.is_null() {
            None
        } else {
            Some(Codec::new(codec))
        }
    }
}

pub struct Codec {
    codec: SharedPtr<ffi::IntegerCODEC>,
}

impl Codec {
    fn new(codec: SharedPtr<ffi::IntegerCODEC>) -> Self {
        Self { codec }
    }

    pub fn encode<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], cxx::Exception> {
        let n = ffi::codec_encode(&self.codec, input, output)?;
        Ok(&mut output[..n])
    }

    pub fn decode<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], cxx::Exception> {
        let n = ffi::codec_decode(&self.codec, input, output)?;
        Ok(&mut output[..n])
    }
}

#[cfg(all(test, feature = "cpp"))]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let factory = CodecFactory::new().unwrap();
        let codec = factory.get_codec("simdfastpfor256").unwrap();

        let input = vec![1, 2, 3, 4, 5];
        let mut output = vec![0; 10];
        let encoded = codec.encode(&input, &mut output).unwrap();

        let mut decoded = vec![0; 10];
        let decoded = codec.decode(encoded, &mut decoded).unwrap();

        assert_eq!(decoded, input);
    }
}
