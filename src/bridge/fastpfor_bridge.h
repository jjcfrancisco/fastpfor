#pragma once

#include <memory>
#include <rust/cxx.h>
#include "codecfactory.h"
#include "codecs.h"

// Adding things to the same namespace as lib to keep things simpler
namespace FastPForLib {

// Instantiate Coder Factory
inline std::unique_ptr<CODECFactory> new_codec_factory() {
    return std::make_unique<CODECFactory>();
}

// Get codec by name from a factory
inline std::shared_ptr<IntegerCODEC> codec_factory_get_from_name(
    const CODECFactory& factory,
    rust::Str name) {
    return factory.getFromName(std::string(name));
}

// Encode helper
inline size_t codec_encode(const std::shared_ptr<IntegerCODEC>& codec,
                         const rust::Slice<const uint32_t> in,
                         rust::Slice<uint32_t> out) {
    size_t outSize = out.size();
    codec->encodeArray(in.data(), in.size(), out.data(), outSize);
    return outSize;
}

// Decode helper: returns consumed input elements
inline size_t codec_decode(const std::shared_ptr<IntegerCODEC>& codec,
                           const rust::Slice<const uint32_t> in,
                           rust::Slice<uint32_t> out) {
    size_t outSize = out.size();
    codec->decodeArray(in.data(), in.size(), out.data(), outSize);
    return outSize;
}

} // namespace FastPForLib
