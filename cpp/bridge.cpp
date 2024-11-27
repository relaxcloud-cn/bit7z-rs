// cpp/bridge.cpp
#include "bridge.hpp"
#include "implementations.hpp"

rust::Vec<KeyValue> cxx_extracting(rust::String path, const rust::Vec<uint8_t> &file)
{
    std::vector<uint8_t> data(file.begin(), file.end());
    std::map<std::string, std::vector<uint8_t>> tmp = extracting(path.c_str(), data);

    rust::Vec<KeyValue> extractingVec;
    for (const auto &item : tmp)
    {
        KeyValue kv;
        kv.key = item.first;
        auto file_data = item.second;
        std::copy(file_data.begin(), file_data.end(), std::back_inserter(kv.value));
        extractingVec.push_back(kv);
    }

    return extractingVec;
}