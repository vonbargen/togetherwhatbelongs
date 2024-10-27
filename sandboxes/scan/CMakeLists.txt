cmake_minimum_required(VERSION 3.15)
project(scan C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_TOOLCHAIN_FILE "${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake")

add_executable(scan scanner.c parser.c main.c)
