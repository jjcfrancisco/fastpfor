#pragma once

#include <memory>
#include <rust/cxx.h>
#include "codecfactory.h"
#include "codecs.h"

// Adding things to the same namespace as lib to keep things simpler
namespace FastPForLib {

// Instantiate Coder Factory
inline std::unique_ptr<FastPForLib::CODECFactory> new_codec_factory() {
    return std::make_unique<FastPForLib::CODECFactory>();
}

// Get codec by name from a factory
inline std::shared_ptr<FastPForLib::IntegerCODEC> codec_factory_get_from_name(
    const FastPForLib::CODECFactory& factory,
    rust::Str name
) {
    return factory.getFromName(std::string(name));
}

} // namespace FastPForLib
