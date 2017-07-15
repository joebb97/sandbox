#include <algorithm>
#include <iostream>
#include <cstdlib>
#include "sorts.h"
using namespace std;
using type = int;
using comp = less<type>;

void print_vec(const vector<type> & vec){
    cout << "{ ";
    for (const auto & i : vec){
        cout << i << " ";
    }
    cout << "}" << endl;
}

void test_sort(void (*sort)(vector<type> &), vector<type> original){
    cout << "Before: ";
    print_vec(original);
    sort(original);
    if (check_sorted<type, comp>(original)) {
        cout << "After: ";
    }
    else {
        cout << "Sort failed!" << endl;
    }
    print_vec(original);
}

void test_sorts(const vector<void (*)(vector<type> &)> & sorts, const vector<type> & original){
    for (const auto sort: sorts){
        test_sort(sort, original);
    }
}

int randint(){
    return rand() % 10000;
}

int main() {
    vector<type> original(10);
    std::srand(std::time(0));
    std::generate(original.begin(), original.end(), randint);
    vector<void (*)(vector<type> &)> sorts = {bubble_sort<type, comp>, insertion_sort<type, comp>};
    test_sorts(sorts, original);
    return 0;
}