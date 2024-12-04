// cpp/bridge.hpp
#pragma once

#include <cxx.h>
#include <string>
#include <cstdint>
#include <vector>

// #include "cxxbridge/bit7z/src/lib.rs.h"

#ifndef CXXBRIDGE1_STRUCT_FilenameAndData
#define CXXBRIDGE1_STRUCT_FilenameAndData
struct FilenameAndData final {
  ::rust::String filename;
  ::rust::Vec<::std::uint8_t> data;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_FilenameAndData

rust::Vec<FilenameAndData> cxx_extract(rust::String path, const rust::Vec<uint8_t>& file, rust::String password);
