cmake_minimum_required(VERSION 3.15)
project(fnr C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_TOOLCHAIN_FILE "${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake")

# Find ICU package
# find_package(ICU COMPONENTS uc i18n io REQUIRED)

add_executable(fnr scanner.c main.c)

