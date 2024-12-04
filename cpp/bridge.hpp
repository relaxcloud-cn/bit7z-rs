// cpp/bridge.hpp
#pragma once

#include <cxx.h>
#include <string>
#include <cstdint>
#include <vector>

// #include "cxxbridge/bit7z/src/lib.rs.h"

struct FilenameAndData;
struct ListData;
struct Item;

#ifndef CXXBRIDGE1_STRUCT_FilenameAndData
#define CXXBRIDGE1_STRUCT_FilenameAndData
struct FilenameAndData final {
  ::rust::String filename;
  ::rust::Vec<::std::uint8_t> data;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_FilenameAndData

#ifndef CXXBRIDGE1_STRUCT_ListData
#define CXXBRIDGE1_STRUCT_ListData
struct ListData final {
  ::std::uint32_t items_count;
  ::std::uint32_t folders_count;
  ::std::uint32_t files_count;
  ::std::uint64_t size;
  ::std::uint64_t packed_size;
  ::rust::Vec<::Item> items;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ListData

#ifndef CXXBRIDGE1_STRUCT_Item
#define CXXBRIDGE1_STRUCT_Item
struct Item final {
  ::std::uint32_t index;
  ::rust::String name;
  ::rust::String extension;
  ::rust::String path;
  bool is_dir;
  ::std::uint64_t size;
  ::std::uint64_t packed_size;
  ::std::uint32_t crc;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_Item


rust::Vec<FilenameAndData> cxx_extract(rust::String path, const rust::Vec<uint8_t>& file, rust::String password);
ListData cxx_list(rust::String path, const rust::Vec<uint8_t>& file);
