// cpp/bridge.cpp
#include "bridge.hpp"
#include "implementations.hpp"

rust::Vec<FilenameAndData> cxx_extract(rust::String path, const rust::Vec<uint8_t> &file, rust::String password)
{
    std::vector<uint8_t> data(file.begin(), file.end());
    std::map<std::string, std::vector<uint8_t>> tmp = extract(path.c_str(), data, password.c_str());

    rust::Vec<FilenameAndData> extractVec;
    for (const auto &item : tmp)
    {
        FilenameAndData kv;
        kv.filename = item.first;
        auto file_data = item.second;
        std::copy(file_data.begin(), file_data.end(), std::back_inserter(kv.data));
        extractVec.push_back(kv);
    }

    return extractVec;
}

ListData cxx_list(rust::String path, const rust::Vec<uint8_t>& file) {
    std::vector<uint8_t> data(file.begin(), file.end());
    return list(path.c_str(), data);
}
