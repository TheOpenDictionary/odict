#include <cstdint>
#include "endian.h"

/**
 * Code taken from
 * https://www.gamedev.net/articles/programming/general-and-gameplay-programming/writing-endian-independent-code-in-c-r2091
 */

short (*big_short)(short s);

short (*little_short)(short s);

int (*big_long)(int i);

int (*little_long)(int i);

uint8_t *(*big_unit8_t)(uint8_t *u, int size);

uint8_t *(*little_uint8_t)(uint8_t *u, int size);

short short_swap(short s) {
    unsigned char b1, b2;

    b1 = s & 255;
    b2 = (s >> 8) & 255;

    return (b1 << 8) + b2;
}

short short_no_swap(short s) {
    return s;
}

int long_swap(int i) {
    unsigned char b1, b2, b3, b4;

    b1 = i & 255;
    b2 = (i >> 8) & 255;
    b3 = (i >> 16) & 255;
    b4 = (i >> 24) & 255;

    return ((int) b1 << 24) + ((int) b2 << 16) + ((int) b3 << 8) + b4;
}

int long_no_swap(int i) {
    return i;
}

float float_swap(float f) {
    union {
        float f;
        unsigned char b[4];
    } dat1, dat2;

    dat1.f = f;
    dat2.b[0] = dat1.b[3];
    dat2.b[1] = dat1.b[2];
    dat2.b[2] = dat1.b[1];
    dat2.b[3] = dat1.b[0];
    return dat2.f;
}

float float_no_swap(float f) {
    return f;
}

uint8_t *uint8_t_swap(uint8_t* u, int size) {
    char *head = (char *)u;
    char *tail = head + size - 1;

    for (; tail > head; --tail, ++head) {
        char temp = *head;
        *head = *tail;
        *tail = temp;
    }

    return u;
}

uint8_t *uint8_t_no_swap(uint8_t* u, int size) {
    return u;
}

EndianTypes::EndianTypes() {}

void EndianTypes::init() {
    uint8_t SwapTest[2] = {1, 0};

    if (*(short *) SwapTest == 1) {
        big_short = short_swap;
        little_short = short_no_swap;
        big_long = long_swap;
        little_long = long_no_swap;
        big_unit8_t = uint8_t_swap;
        little_uint8_t = uint8_t_no_swap;
    } else {
        big_short = short_no_swap;
        little_short = short_swap;
        big_long = long_no_swap;
        little_long = long_swap;
        big_unit8_t = uint8_t_no_swap;
        little_uint8_t = uint8_t_swap;
    }
}