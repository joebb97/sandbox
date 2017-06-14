#include <algorithm>
#include <iostream>
#include "sorts.h"
using namespace std;

template <class T>
void print_vec(const vector<T> & vec){
    cout << "{ ";
    for (const auto & i : vec){
        cout << i << " ";
    }
    cout << "}";
}

template <class T, class Compare=std::less<T>>
void test_sort(void (*sort)(vector<T> &), vector<T> original){
    cout << "Before: ";
    print_vec<T>(original);
    sort(original);
    if (check_sorted<T, Compare>(original)) {
        cout << endl << "After: ";
        print_vec(original);
    }
    else {
        cout << "Sort failed!" << endl;
    }
}

int main() {
    vector<int> original(10);
    std::generate(original.begin(), original.end(), std::rand);
    using type = int;
    using func = greater<type>;
    test_sort<type, func>(bubble_sort<type, func>, original);
    return 0;
}