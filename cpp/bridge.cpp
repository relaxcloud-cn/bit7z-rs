// cpp/bridge.cpp
#include "bridge.hpp"
#include "implementations.hpp"

rust::Vec<FilenameAndData> cxx_extracting(rust::String path, const rust::Vec<uint8_t> &file, rust::String password)
{
    std::vector<uint8_t> data(file.begin(), file.end());
    std::map<std::string, std::vector<uint8_t>> tmp = extracting(path.c_str(), data, password.c_str());

    rust::Vec<FilenameAndData> extractingVec;
    for (const auto &item : tmp)
    {
        FilenameAndData kv;
        kv.filename = item.first;
        auto file_data = item.second;
        std::copy(file_data.begin(), file_data.end(), std::back_inserter(kv.data));
        extractingVec.push_back(kv);
    }

    return extractingVec;
}