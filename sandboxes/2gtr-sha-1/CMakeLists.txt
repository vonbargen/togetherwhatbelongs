cmake_minimum_required(VERSION 3.29)
project(2gtr-sha1 C)

set(CMAKE_C_STANDARD 11)

find_package(OpenSSL REQUIRED)

add_executable(2gtr-sha1 main.c
        sha-1.c
        sha-1.h)

target_include_directories(2gtr-sha1 PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(2gtr-sha1 PRIVATE OpenSSL::Crypto)
