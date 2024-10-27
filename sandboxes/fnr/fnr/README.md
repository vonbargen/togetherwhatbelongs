## How to build
```
mkdir build
cd build
cmake ..
make
```
oder 
```
cmake --build .
cmake --build . -j 4
```
## How to run
```
./fnr 
```

Build a release:

```
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --config Release
```
   


