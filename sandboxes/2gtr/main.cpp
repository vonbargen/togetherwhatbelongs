#include "sha1.h"
#include <fmt/core.h>

int main() {
    fmt::print("Hello, World!\n");

    std::string filename = "../main.cpp";
    std::string hash = SHA1::hashFile(filename);

    if (!hash.empty()) {
        fmt::print("SHA1 hash of {}: {}\n", filename, hash);
    } else {
        fmt::print("Failed to calculate hash for {}\n", filename);
    }

    return 0;
}
