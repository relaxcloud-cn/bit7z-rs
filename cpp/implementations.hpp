#pragma once

#include <string>
#include <map>
#include <iostream>
#include <vector>
#include <bit7z/bitfileextractor.hpp>
#include <bit7z/bit7zlibrary.hpp>
#include <bit7z/bitexception.hpp>
#include <bit7z/bitarchivereader.hpp>
#include "bit7z/biterror.hpp"
#include <stdexcept> 

std::map<std::string, std::vector<uint8_t>> extracting(std::string path, const std::vector<uint8_t> file, std::string password);
