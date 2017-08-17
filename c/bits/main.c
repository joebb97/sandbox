#include <stdio.h>
#include <limits.h>

void display_in_bin(unsigned long n) {
    unsigned long shifts = 0;
    for (unsigned long i = 1 << sizeof(n) * CHAR_BIT - 1; shifts < sizeof(n) * 8; i = i >> 1, ++shifts) {
        (n & i) ? printf("1"): printf("0");
    }
    printf("\n");
}

void print_radices(long n) {
    printf("Decimal: %ld, Hex: %lx, Octal: %lo, Binary: ", n, n, n);
    display_in_bin(n);
    printf("\n");
}

long circular_shift(long x, int shift) {
    return (x << shift) | (x >> (sizeof(x)* 8 - shift));
}

void get_input(long * x, long * y, long * shift) {
    scanf("%ld %ld %ld", x, y, shift);
}

int main() {
    // long x, y, shift;
    // printf("Please enter in 3 numbers: ");
    // get_input(&x, &y, &shift);
    // print_radices(x);
    // print_radices(y);
    long x = 34;
    display_in_bin(x);
    return 0;
}
