#include <stdio.h>
#include <limits.h>

void display_in_bin(unsigned n) {
    unsigned shifts = 0;
    for (unsigned i = 1 << (sizeof(n) * CHAR_BIT - 1); shifts < sizeof(n) * CHAR_BIT; i = i >> 1, ++shifts) {
        (n & i) ? printf("1"): printf("0");
    }
    printf("\n");
}

void print_radices(int n) {
    printf("Decimal: %d, Hex: %x, Octal: %o, Binary: ", n, n, n);
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

void prog1(){
    int x, y, shift;
    printf("Please enter in 3 numbers: ");
    get_input(&x, &y, &shift);
    print_radices(x);
    print_radices(y);
    printf("2nd number left circular shifted %d places\n", shift);
    print_radices(left_circular_shift(y, shift));
}

void prog2(){
    int x = 0xFFFF0000;
    print_radices(x);
    print_radices(x / 4);
}

int main() {
    //prog1();
    prog2();
}
