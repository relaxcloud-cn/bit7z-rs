#include "implementations.hpp"

std::map<std::string, std::vector<uint8_t>> extracting(std::string path, const std::vector<uint8_t> file)
{
    std::locale::global(std::locale("en_US.UTF-8"));
    std::map<std::string, std::vector<uint8_t>> result_map;
    try
    {
        using namespace bit7z;
        Bit7zLibrary lib{path};
        BitExtractor<std::vector<uint8_t>> extractor{lib, BitFormat::Auto};
        extractor.extract(file, result_map);
    }
    catch (const bit7z::BitException &ex)
    {
        std::cerr << "Error occurred: " << ex.what() << std::endl;
    }
    catch (const std::exception &ex)
    {
        std::cerr << "Standard exception: " << ex.what() << std::endl;
    }
    catch (...)
    {
        std::cerr << "Unknown error occurred!" << std::endl;
    }

    return result_map;
}
