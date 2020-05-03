#include "bits.h"
#include <stdio.h>

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
    int x = 0777;
    print_radices(x);
    print_radices(x / 4);
}

int main() {
    //prog1();
    prog2();
    elsewhere();
}
