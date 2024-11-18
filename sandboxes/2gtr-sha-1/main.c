#include <stdio.h>
#include "sha-1.h"

int main(void) {

    printf("Hello, World!\n");

    const char *filename = "../main.c";  // You can change this to any file you want to hash
    uint8_t hash[SHA1_HASH_SIZE];

    if (compute_file_sha1(filename, hash) == 0) {
        printf("SHA-1 hash of %s: ", filename);
        print_sha1_hash(hash);
    } else {
        printf("Failed to compute SHA-1 hash for %s\n", filename);
    }

    return 0;
}