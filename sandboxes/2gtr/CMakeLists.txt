cmake_minimum_required(VERSION 3.29)
project(2gtr C)

set(CMAKE_C_STANDARD 11)

find_package(OpenSSL REQUIRED)

add_executable(2gtr main.c
        ssh-1.c
        ssh-1.h)

target_include_directories(2gtr PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(2gtr PRIVATE OpenSSL::Crypto)
