// cpp/bridge.hpp
#pragma once

#include <cxx.h>
#include <string>
#include <cstdint>
#include <vector>

// #include "cxxbridge/archive_extractor/src/lib.rs.h"
#ifndef CXXBRIDGE1_STRUCT_KeyValue
#define CXXBRIDGE1_STRUCT_KeyValue
struct KeyValue final {
  ::rust::String key;
  ::rust::Vec<::std::uint8_t> value;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_KeyValue

rust::Vec<KeyValue> cxx_extracting(rust::String path, const rust::Vec<uint8_t>& file);
