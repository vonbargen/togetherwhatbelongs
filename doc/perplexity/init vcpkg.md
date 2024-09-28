Answer
To set up a C++ project using fmt with vcpkg and CMake, follow these steps:
Step 1: Install vcpkg
First, you need to install vcpkg. Clone the vcpkg repository from GitHub:
text
git clone https://github.com/microsoft/vcpkg.git

Navigate to the vcpkg directory and bootstrap it:
text
cd vcpkg
bootstrap-vcpkg.bat # On Windows
./bootstrap-vcpkg.sh # On Linux/Mac

Step 2: Install fmt using vcpkg
Install the fmt library using vcpkg:
text
vcpkg install fmt

Step 3: Create CMakeLists.txt
Create a CMakeLists.txt file in your project directory with the following content:
text
cmake_minimum_required(VERSION 3.10)
project(HelloWorld)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Specify vcpkg toolchain file
set(CMAKE_TOOLCHAIN_FILE "${CMAKE_SOURCE_DIR}/vcpkg/scripts/buildsystems/vcpkg.cmake")

# Find the fmt package
find_package(fmt REQUIRED)

# Add the executable
add_executable(HelloWorld main.cpp)

# Link with fmt
target_link_libraries(HelloWorld PRIVATE fmt::fmt)

Step 4: Write Your C++ Code
Create a main.cpp file in the same directory with your C++ code. Here's an example that reads a file and checks if it contains 'Hello World':
cpp
#include <cstdio> // For FILE, fopen, fread, fclose
#include <fmt/core.h> // For fmt::print

int main() {
    const char* filename = "example.txt"; // File to read
    const char* targetContent = "Hello World"; // Content to find

    // Open the file
    FILE* file = fopen(filename, "r");
    if (!file) {
        fmt::print("Failed to open file {}\n", filename);
        return 1;
    }

    // Read the file content
    char buffer[1024];
    size_t bytesRead = fread(buffer, 1, sizeof(buffer), file);
    fclose(file);

    // Check if the file contains the target content
    bool found = false;
    for (size_t i = 0; i < bytesRead; ++i) {
        if (buffer[i] == targetContent[0]) {
            size_t j;
            for (j = 0; j < sizeof(targetContent) - 1 && i + j < bytesRead; ++j) {
                if (buffer[i + j] != targetContent[j]) break;
            }
            if (j == sizeof(targetContent) - 1) {
                found = true;
                break;
            }
        }
    }

    if (found) {
        fmt::print("File {} contains '{}'.\n", filename, targetContent);
    } else {
        fmt::print("File {} does not contain '{}'.\n", filename, targetContent);
    }

    return 0;
}

Step 5: Configure and Build
Create a build directory and navigate to it:
text
mkdir build
cd build

Configure the project using CMake:
text
cmake ..

Build the project:
text
cmake --build .

Run the executable:
text
./HelloWorld

This setup uses vcpkg to manage dependencies and CMake to build the project, ensuring that the fmt library is properly integrated.
SONAR HUGE
How do I configure vcpkg to use the fmt library in my CMake project







Sources
Answer
To configure vcpkg to use the fmt library in your CMake project, follow these steps:
Install vcpkg:
Clone the vcpkg repository from GitHub:
text
git clone https://github.com/microsoft/vcpkg.git

Navigate to the vcpkg directory and bootstrap it:
text
cd vcpkg
./bootstrap-vcpkg.sh # On Linux/Mac
bootstrap-vcpkg.bat # On Windows

Install fmt using vcpkg:
Install the fmt library using vcpkg:
text
./vcpkg install fmt

Create a vcpkg Manifest File:
Navigate to your project directory and create a vcpkg.json manifest file:
text
vcpkg new --application

Add the fmt dependency to the manifest file:
text
vcpkg add port fmt

Your vcpkg.json should now contain:
json
{
  "dependencies": [
    "fmt"
  ]
}

Configure CMake:
Create a CMakeLists.txt file in your project directory with the following content:
text
cmake_minimum_required(VERSION 3.10)
project(HelloWorld)
find_package(fmt CONFIG REQUIRED)
add_executable(HelloWorld main.cpp)
target_link_libraries(HelloWorld PRIVATE fmt::fmt)

This CMakeLists.txt file specifies the minimum CMake version, sets the project name, finds the fmt package, adds an executable target, and links it against the fmt library.
Set Up CMake Presets:
Create a CMakePresets.json file in your project directory with the following content:
json
{
  "version": 2,
  "configurePresets": [
    {
      "name": "vcpkg",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build",
      "cacheVariables": {
        "CMAKE_TOOLCHAIN_FILE": "$env{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake"
      }
    }
  ]
}

Create a CMakeUserPresets.json file with the following content, replacing <path to vcpkg> with the actual path to your vcpkg installation:
json
{
  "version": 2,
  "configurePresets": [
    {
      "name": "default",
      "inherits": "vcpkg",
      "environment": {
        "VCPKG_ROOT": "<path to vcpkg>"
      }
    }
  ]
}

Build Your Project:
Configure your project using CMake:
text
cmake --preset=default

Build your project:
text
cmake --build build

By following these steps, you will have successfully configured vcpkg to use the fmt library in your CMake project.
