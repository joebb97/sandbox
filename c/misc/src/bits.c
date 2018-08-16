#include <stdio.h>
#include <limits.h>
#include "bits.h"

void display_in_bin(unsigned n) {
    unsigned shifts = 0;
    unsigned num_bits = sizeof(n) * CHAR_BIT;
    for (unsigned i = 1 << (num_bits - 1); shifts < num_bits; i = i >> 1, ++shifts) {
        (n & i) ? printf("1"): printf("0");
    }
    printf("\n");
}

void print_radices(int n) {
    printf("Decimal (signed): %d\n"
           "Decimal (unsigned): %u\n"
           "Hex: %x\n" 
           "Octal: %o\n"
           "Binary: " , n, n, n, n);
    display_in_bin(n);
    printf("\n");
}

int left_circular_shift(int x, int shift) {
    return (x << shift) | (x >> (sizeof(x) * CHAR_BIT - shift));
}

int right_circular_shift(int x, int shift){
    return 0;
}

void get_input(int * x, int * y, int * shift) {
    scanf("%d %d %d", x, y, shift);
}

void elsewhere(){
    printf("I'm from somewhere else\n");
}
