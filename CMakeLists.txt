cmake_minimum_required(VERSION 3.10)
project(HelloWorld)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Find the fmt package
find_package(fmt CONFIG REQUIRED)

# Add the executable
add_executable(HelloWorld main.cpp)

# Link with fmt
target_link_libraries(HelloWorld PRIVATE fmt::fmt)
