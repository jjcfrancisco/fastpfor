use cxx::{SharedPtr, UniquePtr};

#[cxx::bridge(namespace = "FastPForLib")]
mod ffi {
    unsafe extern "C++" {
        include!("fastpfor_bridge.h");

        type CODECFactory;
        type IntegerCODEC;

        fn new_codec_factory() -> UniquePtr<CODECFactory>;

        fn codec_factory_all_names(factory: &CODECFactory) -> UniquePtr<CxxVector<CxxString>>;

        fn codec_factory_get_from_name(
            factory: &CODECFactory,
            name: &str,
        ) -> SharedPtr<IntegerCODEC>;

        // Encode methods: returns number of output values

        fn codec_encode32(
            codec: &SharedPtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u32],
        ) -> Result<usize>;

        fn codec_encode64(
            codec: &SharedPtr<IntegerCODEC>,
            input: &[u64],
            output: &mut [u32],
        ) -> Result<usize>;

        // Decode methods: returns consumed input values

        fn codec_decode32(
            codec: &SharedPtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u32],
        ) -> Result<usize>;

        fn codec_decode64(
            codec: &SharedPtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u64],
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

    /// Returns a list of all codec names.
    pub fn all_names(&self) -> Vec<String> {
        let names = ffi::codec_factory_all_names(&self.factory);
        names.iter().map(|s| s.to_string()).collect()
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
    /// Creates a new `Codec` instance.
    fn new(codec: SharedPtr<ffi::IntegerCODEC>) -> Self {
        Self { codec }
    }

    /// Encode a slice of 32-bit integers.
    pub fn encode32<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], cxx::Exception> {
        let n = ffi::codec_encode32(&self.codec, input, output)?;
        Ok(&mut output[..n])
    }

    /// Encode a slice of 64-bit integers.
    pub fn encode64<'out>(
        &self,
        input: &[u64],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], cxx::Exception> {
        let n = ffi::codec_encode64(&self.codec, input, output)?;
        Ok(&mut output[..n])
    }

    /// Decode a slice of 32-bit integers.
    pub fn decode32<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], cxx::Exception> {
        let n = ffi::codec_decode32(&self.codec, input, output)?;
        Ok(&mut output[..n])
    }

    /// Decode a slice of 64-bit integers.
    pub fn decode64<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u64],
    ) -> Result<&'out mut [u64], cxx::Exception> {
        let n = ffi::codec_decode64(&self.codec, input, output)?;
        Ok(&mut output[..n])
    }
}

#[cfg(all(test, feature = "cpp"))]
mod tests {
    use super::*;

    #[test]
    fn test_bp32() {
        full_test_32("BP32");
        // full_test_64("BP32");
    }
    #[test]
    fn test_copy() {
        full_test_32("copy");
        // full_test_64("copy");
    }
    #[test]
    fn test_fastbinarypacking16() {
        full_test_32("fastbinarypacking16");
        // full_test_64("fastbinarypacking16");
    }
    #[test]
    fn test_fastbinarypacking32() {
        full_test_32("fastbinarypacking32");
        // full_test_64("fastbinarypacking32");
    }
    #[test]
    fn test_fastbinarypacking8() {
        full_test_32("fastbinarypacking8");
        // full_test_64("fastbinarypacking8");
    }
    #[test]
    fn test_fastpfor128() {
        full_test_32("fastpfor128");
        full_test_64("fastpfor128");
    }
    #[test]
    fn test_fastpfor256() {
        full_test_32("fastpfor256");
        full_test_64("fastpfor256");
    }
    #[test]
    fn test_maskedvbyte() {
        full_test_32("maskedvbyte");
        // full_test_64("maskedvbyte");
    }
    #[test]
    fn test_newpfor() {
        full_test_32("newpfor");
        // full_test_64("newpfor");
    }
    #[test]
    fn test_optpfor() {
        full_test_32("optpfor");
        // full_test_64("optpfor");
    }
    #[test]
    fn test_pfor() {
        full_test_32("pfor");
        // full_test_64("pfor");
    }
    #[test]
    fn test_pfor2008() {
        full_test_32("pfor2008");
        // full_test_64("pfor2008");
    }
    #[test]
    fn test_simdbinarypacking() {
        full_test_32("simdbinarypacking");
        // full_test_64("simdbinarypacking");
    }
    #[test]
    fn test_simdfastpfor128() {
        full_test_32("simdfastpfor128");
        // full_test_64("simdfastpfor128");
    }
    #[test]
    fn test_simdfastpfor256() {
        full_test_32("simdfastpfor256");
        // full_test_64("simdfastpfor256");
    }
    #[test]
    fn test_simdgroupsimple() {
        full_test_32("simdgroupsimple");
        // full_test_64("simdgroupsimple");
    }
    #[test]
    fn test_simdgroupsimple_ringbuf() {
        full_test_32("simdgroupsimple_ringbuf");
        // full_test_64("simdgroupsimple_ringbuf");
    }
    #[test]
    fn test_simdnewpfor() {
        full_test_32("simdnewpfor");
        // full_test_64("simdnewpfor");
    }
    #[test]
    fn test_simdoptpfor() {
        full_test_32("simdoptpfor");
        // full_test_64("simdoptpfor");
    }
    #[test]
    fn test_simdpfor() {
        full_test_32("simdpfor");
        // full_test_64("simdpfor");
    }
    #[test]
    fn test_simdsimplepfor() {
        full_test_32("simdsimplepfor");
        // full_test_64("simdsimplepfor");
    }
    #[test]
    fn test_simple16() {
        full_test_32("simple16");
        // full_test_64("simple16");
    }
    #[test]
    fn test_simple8b() {
        full_test_32("simple8b");
        // full_test_64("simple8b");
    }
    #[test]
    fn test_simple8b_rle() {
        full_test_32("simple8b_rle");
        // full_test_64("simple8b_rle");
    }
    #[test]
    fn test_simple9() {
        full_test_32("simple9");
        // full_test_64("simple9");
    }
    #[test]
    fn test_simple9_rle() {
        full_test_32("simple9_rle");
        // full_test_64("simple9_rle");
    }
    #[test]
    fn test_simplepfor() {
        full_test_32("simplepfor");
        // full_test_64("simplepfor");
    }
    #[test]
    fn test_streamvbyte() {
        full_test_32("streamvbyte");
        // full_test_64("streamvbyte");
    }
    #[test]
    fn test_varint() {
        full_test_32("varint");
        full_test_64("varint");
    }
    #[test]
    fn test_varintg8iu() {
        full_test_32("varintg8iu");
        // full_test_64("varintg8iu");
    }
    #[test]
    fn test_varintgb() {
        full_test_32("varintgb");
        // full_test_64("varintgb");
    }
    #[test]
    fn test_vbyte() {
        full_test_32("vbyte");
        // full_test_64("vbyte");
    }

    // This leaks memory!
    // #[test]
    // fn test_vsencoding() {
    //     full_test_32("vsencoding");
    //     full_test_64("vsencoding");
    // }

    fn full_test_32(name: &str) {
        let factory = CodecFactory::new().unwrap();
        let codec = factory.get_codec(name).unwrap();
        roundtrip_32(&codec);
    }

    fn full_test_64(name: &str) {
        let factory = CodecFactory::new().unwrap();
        let codec = factory.get_codec(name).unwrap();
        roundtrip_64(&codec);
    }

    fn roundtrip_32(codec: &Codec) {
        let input = vec![1, 2, 3, 4, 5];
        let mut output = vec![0; 10];
        let encoded = codec.encode32(&input, &mut output).unwrap();
        let mut decoded = vec![0; 10];
        let decoded = codec.decode32(encoded, &mut decoded).unwrap();
        assert_eq!(decoded, input);
    }

    fn roundtrip_64(codec: &Codec) {
        let input = vec![1, 2, 3, 4, 5];
        let mut output = vec![0; 10];
        let encoded = codec.encode64(&input, &mut output).unwrap();

        let mut decoded = vec![0; 10];
        let decoded = codec.decode64(encoded, &mut decoded).unwrap();
        assert_eq!(decoded, input);
    }
}
