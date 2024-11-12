// sha1.cpp
#include "sha1.h"
#include <fstream>
#include <vector>
#include <fmt/format.h>
#include <fmt/ranges.h> // Add this line
#include <openssl/evp.h>

std::string SHA1::hashFile(const std::string& filename) {
    std::ifstream file(filename, std::ios::binary);
    if (!file) {
        return "";
    }

    EVP_MD_CTX* ctx = EVP_MD_CTX_new();
    if (!ctx) {
        return "";
    }

    if (EVP_DigestInit_ex(ctx, EVP_sha1(), nullptr) != 1) {
        EVP_MD_CTX_free(ctx);
        return "";
    }

    std::vector<unsigned char> buffer(BUFFER_SIZE);
    while (file.good()) {
        file.read(reinterpret_cast<char*>(buffer.data()), buffer.size());
        if (EVP_DigestUpdate(ctx, buffer.data(), file.gcount()) != 1) {
            EVP_MD_CTX_free(ctx);
            return "";
        }
    }

    unsigned char hash[EVP_MAX_MD_SIZE];
    unsigned int hash_len;
    if (EVP_DigestFinal_ex(ctx, hash, &hash_len) != 1) {
        EVP_MD_CTX_free(ctx);
        return "";
    }

    EVP_MD_CTX_free(ctx);

    // Use a different approach to format the hash
    return fmt::format("{:02x}", fmt::join(std::vector<unsigned char>(hash, hash + hash_len), ""));
}
