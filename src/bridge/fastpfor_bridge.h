#pragma once

#include <memory>
#include <rust/cxx.h>
#include "codecfactory.h"
#include "codecs.h"

// Adding things to the same namespace as lib to keep things simpler
namespace FastPForLib {

// Encode helpers
inline size_t codec_encode32(
        const std::unique_ptr<IntegerCODEC>& codec,
        const rust::Slice<const uint32_t> in,
        rust::Slice<uint32_t> out
) {
    size_t outSize = out.size();
    codec->encodeArray(in.data(), in.size(), out.data(), outSize);
    return outSize;
}

inline size_t codec_encode64(
        const std::unique_ptr<IntegerCODEC>& codec,
        const rust::Slice<const uint64_t> in,
        rust::Slice<uint32_t> out
) {
    size_t outSize = out.size();
    codec->encodeArray(in.data(), in.size(), out.data(), outSize);
    return outSize;
}

// Decode helper: returns consumed input elements
inline size_t codec_decode32(
        const std::unique_ptr<IntegerCODEC>& codec,
        const rust::Slice<const uint32_t> in,
        rust::Slice<uint32_t> out
) {
    size_t outSize = out.size();
    codec->decodeArray(in.data(), in.size(), out.data(), outSize);
    return outSize;
}

inline size_t codec_decode64(
        const std::unique_ptr<IntegerCODEC>& codec,
        const rust::Slice<const uint32_t> in,
        rust::Slice<uint64_t> out
) {
    size_t outSize = out.size();
    codec->decodeArray(in.data(), in.size(), out.data(), outSize);
    return outSize;
}

} // namespace FastPForLib
