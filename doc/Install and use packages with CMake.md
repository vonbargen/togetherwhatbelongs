https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash

rm -rf build
mkdir build

cmake --preset=default
cmake --build build
./build/HelloWorld