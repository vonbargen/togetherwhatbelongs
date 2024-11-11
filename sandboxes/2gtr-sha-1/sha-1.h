//
// Created by thorsten on 28.10.24.
//
// ssh_1.h
#ifndef SSH_1_H
#define SSH_1_H

#include <stdint.h>
#include <stdio.h>

#define SHA1_HASH_SIZE 20

// Function to compute SHA-1 hash of a file
int compute_file_sha1(const char *filename, uint8_t hash[SHA1_HASH_SIZE]);

// Function to print SHA-1 hash as a hexadecimal string
void print_sha1_hash(const uint8_t hash[SHA1_HASH_SIZE]);

#endif //SSH_1_H
