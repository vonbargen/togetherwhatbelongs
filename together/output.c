#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <stdint.h>

// Forward declarations
int64_t oberon_Add(int64_t oberon_a, int64_t oberon_b);
void oberon_Init(void);
void oberon_WriteInt(int64_t oberon_n);
void oberon_WriteLn(void);

// Constants
#define oberon_MaxSize 100LL
#define oberon_Pi 3.14159

// Type definitions
typedef int64_t oberon_IntArray
[oberon_MaxSize];
typedef struct {
    double oberon_x;
    double oberon_y;
} oberon_Point;

// Global variables
int64_t oberon_count;
oberon_Point oberon_points[10];

int64_t oberon_Add(int64_t oberon_a, int64_t oberon_b) {
    return (oberon_a + oberon_b);
}

void oberon_Init(void) {
    int64_t oberon_i;
    oberon_count = 0LL;
    for (oberon_i = 0LL; oberon_i <= 9LL; oberon_i += 1) {
        oberon_points[oberon_i].oberon_x = 0;
        oberon_points[oberon_i].oberon_y = 0;
    }
}

void oberon_WriteInt(int64_t oberon_n) {
}

void oberon_WriteLn(void) {
}

int main(void) {
    oberon_Init();
    oberon_count = oberon_Add(5LL, 37LL);
    oberon_WriteInt(oberon_count);
    oberon_WriteLn();
    return 0;
}
