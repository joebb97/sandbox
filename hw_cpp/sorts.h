#pragma once

#include <vector>
#include <functional>
#include <utility>

template <class T, class Compare=std::less<T>>
void comp_swap(T & left, T & right, Compare c = Compare()){
    if (c(left,right)){
        std::swap(left, right);
    }
}

template <class T, class Compare=std::less<T>>
bool check_sorted(const std::vector<T> & vec, Compare comp = Compare()){
    for (auto it = vec.begin(); it != vec.end() - 1; ++it){
        if (!comp(*it, *(it + 1))){
            return false;
        }
    }
    return true;
}

template <class T, class Compare=std::less<T>>
void bubble_sort(std::vector<T> & unsorted){
    for (auto i = unsorted.begin(); i != unsorted.end(); ++i){
        for (auto j = unsorted.begin(); j != unsorted.end() - 1; ++j){
            comp_swap<T, Compare>(*i, *j);
        }
    }
}