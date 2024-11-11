#include "sha-1.h"
#include <openssl/evp.h>
#include <stdio.h>

#define BUFFER_SIZE 8192

int compute_file_sha1(const char *filename, uint8_t hash[SHA1_HASH_SIZE]) {
    FILE *file = fopen(filename, "rb");
    if (!file) {
        perror("Error opening file");
        return -1;
    }

    EVP_MD_CTX *mdctx = EVP_MD_CTX_new();
    if (!mdctx) {
        fclose(file);
        return -1;
    }

    if (EVP_DigestInit_ex(mdctx, EVP_sha1(), NULL) != 1) {
        EVP_MD_CTX_free(mdctx);
        fclose(file);
        return -1;
    }

    unsigned char buffer[BUFFER_SIZE];
    size_t bytes_read;

    while ((bytes_read = fread(buffer, 1, BUFFER_SIZE, file)) > 0) {
        if (EVP_DigestUpdate(mdctx, buffer, bytes_read) != 1) {
            EVP_MD_CTX_free(mdctx);
            fclose(file);
            return -1;
        }
    }

    unsigned int digest_len;
    if (EVP_DigestFinal_ex(mdctx, hash, &digest_len) != 1) {
        EVP_MD_CTX_free(mdctx);
        fclose(file);
        return -1;
    }

    EVP_MD_CTX_free(mdctx);
    fclose(file);
    return 0;
}

void print_sha1_hash(const uint8_t hash[SHA1_HASH_SIZE]) {
    for (int i = 0; i < SHA1_HASH_SIZE; i++) {
        printf("%02x", hash[i]);
    }
    printf("\n");
}
