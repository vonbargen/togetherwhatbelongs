cmake_minimum_required(VERSION 3.29)
project(2gtr)

set(CMAKE_CXX_STANDARD 20)

find_package(fmt CONFIG REQUIRED)
find_package(OpenSSL REQUIRED)

add_executable(2gtr 
	main.cpp
		sha-1.c
		sha-1.h
)

target_include_directories(2gtr PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(2gtr PRIVATE
        OpenSSL::Crypto
        fmt::fmt
)
