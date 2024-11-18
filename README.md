# Together what belongs Together
	- language / documentation and program
	- git / collaboration and database
	- organisation / use defaults like maven 
- do not reinvent the wheel!
- use highly abstraction layer
- be fast, be funny
- Feature
  - More than one Repo / Soruces
  - Likes
  - Visit Count
  - Fulltext like glimpse
  - Git Index / but no Repo


## GitHub
git remote add origin https://github.com/vonbargen/togetherwhatbelongs.git

## MasOS

 brew install autoconf automake autoconf-archive

## Build

### Environment
​	```sh
export CMAKE_PATH=$HOME/Applications/CLion.app/Contents/bin/cmake/mac/aarch64/bin
export VCPKG_ROOT=$HOME/.vcpkg-clion/vcpkg
export CMAKE_TOOLCHAIN_FILE=$VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake
export PATH=$CMAKE_PATH:$VCPKG_ROOT:$PATH
​	```

https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash

- rm -rf build
- mkdir build
- cmake --preset=default
- cmake --build build
- /build/HelloWorld

https://github.com/kaishin/markoff.git
Ich möchte mit hilfe von commonMark markdown dateien parsen, anreichern und sie mit webkit in einer App ausgeben. Kann man so eine app schreiben?
