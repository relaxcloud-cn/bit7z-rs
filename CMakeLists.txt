cmake_minimum_required(VERSION 3.15)
project(bit7z_example)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(unofficial-bit7z CONFIG REQUIRED)

add_executable(bit7z_example main.cpp)

target_link_libraries(bit7z_example 
    PRIVATE 
        unofficial::bit7z::bit7z64
)