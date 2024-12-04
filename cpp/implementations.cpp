#include "implementations.hpp"

std::map<std::string, std::vector<uint8_t>> extract(std::string path, const std::vector<uint8_t> file, std::string password)
{
    std::locale::global(std::locale("en_US.UTF-8"));
    std::map<std::string, std::vector<uint8_t>> result_map;
    using namespace bit7z;
    try
    {
        Bit7zLibrary lib{path};
        BitExtractor<std::vector<uint8_t>> extractor{lib, BitFormat::Auto};
        if (!password.empty())
        {
            extractor.setPassword(password);
        }
        extractor.extract(file, result_map);
    }
    catch (const bit7z::BitException &ex)
    {
        std::error_code ec = ex.code();
        switch(static_cast<BitError>(ec.value())) {
            case BitError::Fail:
                std::cerr << "Operation failed" << std::endl;
                break;
            case BitError::FilterNotSpecified:
                std::cerr << "Filter not specified" << std::endl;
                break;
            case BitError::FormatFeatureNotSupported:
                std::cerr << "Format feature not supported" << std::endl;
                break;
            case BitError::IndicesNotSpecified:
                std::cerr << "Indices not specified" << std::endl;
                break;
            case BitError::InvalidArchivePath:
                std::cerr << "Invalid archive path" << std::endl;
                break;
            case BitError::InvalidOutputBufferSize:
                std::cerr << "Invalid output buffer size" << std::endl;
                break;
            case BitError::InvalidCompressionMethod:
                std::cerr << "Invalid compression method" << std::endl;
                break;
            case BitError::InvalidDictionarySize:
                std::cerr << "Invalid dictionary size" << std::endl;
                break;
            case BitError::InvalidIndex:
                std::cerr << "Invalid index" << std::endl;
                break;
            case BitError::InvalidWordSize:
                std::cerr << "Invalid word size" << std::endl;
                break;
            case BitError::ItemIsAFolder:
                std::cerr << "Item is a folder" << std::endl;
                break;
            case BitError::ItemMarkedAsDeleted:
                std::cerr << "Item marked as deleted" << std::endl;
                break;
            case BitError::NoMatchingItems:
                std::cerr << "No matching items found" << std::endl;
                break;
            case BitError::NoMatchingSignature:
                std::cerr << "No matching signature" << std::endl;
                break;
            case BitError::NonEmptyOutputBuffer:
                std::cerr << "Non-empty output buffer" << std::endl;
                break;
            case BitError::NullOutputBuffer:
                std::cerr << "Null output buffer" << std::endl;
                break;
            case BitError::RequestedWrongVariantType:
                std::cerr << "Requested wrong variant type" << std::endl;
                break;
            case BitError::UnsupportedOperation:
                std::cerr << "Unsupported operation" << std::endl;
                break;
            case BitError::UnsupportedVariantType:
                std::cerr << "Unsupported variant type" << std::endl;
                break;
            case BitError::WrongUpdateMode:
                std::cerr << "Wrong update mode" << std::endl;
                break;
            case BitError::InvalidZipPassword:
                std::cerr << "Invalid ZIP password" << std::endl;
                break;
            default:
                std::cerr << "Unknown bit7z error: " << ex.what() << std::endl;
        }
        throw; // Re-throw exception
    }
    catch (const std::exception &ex)
    {
        std::cerr << "Standard exception: " << ex.what() << std::endl;
        throw; // Re-throw exception
    }
    catch (...)
    {
        std::cerr << "Unknown error occurred!" << std::endl;
        throw; // Re-throw exception
    }

    return result_map;
}