#pragma once

#include <vector>
#include <functional>
#include <utility>
#include <limits>

template <class T, class Compare=std::less<T>>
void comp_swap(T & left, T & right, Compare c = Compare()){
    if (c(right,left)){
        std::swap(left, right);
    }
}

template <class T, class Compare=std::less<T>>
bool bubble_comp_swap(T & left, T & right, Compare c = Compare()){
    if (c(right,left)){
        std::swap(left, right);
        return true;
    }
    return false;
}

template <class T, class Compare=std::less<T>>
bool check_sorted(const std::vector<T> & vec, Compare comp = Compare()){
    for (auto it = vec.begin(); it != vec.end() - 1; ++it){
        if (comp(*(it + 1), *it)){
            return false;
        }
    }
    return true;
}

template <class T, class Compare=std::less<T>>
void bubble_sort(std::vector<T> & unsorted){
    bool swapped = false;
    for (auto i = unsorted.begin(); i != unsorted.end(); ++i){
        for (auto j = unsorted.begin(); j != unsorted.end() - 1; ++j){
            if (bubble_comp_swap<T, Compare>(*j, *(j+1))){
                swapped = true;
            }
        }
        if (!swapped){
            break;
        }
        swapped = false;
    }
}

template <class T, class Compare=std::less<T>>
void insertion_sort(std::vector<T> & unsorted){
    for (auto i = unsorted.begin() + 1; i != unsorted.end(); ++i){
        for (auto j = i; j != unsorted.begin(); --j){
            comp_swap<T, Compare>(*(j-1), *j);
        }
    }
}

template <class T, class Compare=std::less<T>>
void selection_sort(std::vector<T> & unsorted){
    for (auto i = unsorted.begin(); i != unsorted.end(); ++i){

    }
}