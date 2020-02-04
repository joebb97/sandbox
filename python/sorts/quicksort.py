#!/usr/bin/env python
"""Quicksort implementation."""
import random

# import logging
# from logging import (debug as dprint, info as iprint)
from sorts import check_sorted

# logging.basicConfig(level=logging.INFO)
# XXX: Logging slows this boy DOWN, also this things doesn't even
# complete for inputs 10^6


def __swap_list_elts(in_list, elt1, elt2):
    """Swap two list elements wrapper."""
    in_list[elt1], in_list[elt2] = in_list[elt2], in_list[elt1]


def __partition(in_list, left, right):
    """Partition in_list into elts < pivot and elts > pivot.

    in_list: list to be partitioned range is [left, right)
    left: left index of list
    right: right index past the list
    returns: the new index of the pivot
    """
    pivot = random.randint(left, right - 1)
    # dprint(f"---\nPARTITIONING {in_list} at pivot_val: {in_list[pivot]}")
    in_list[:] = in_list  # Modify in place
    right -= 1  # N - 1 is the end index
    # Swap Right and Pivot, update pivot idx to be at the end
    __swap_list_elts(in_list, right, pivot)
    pivot = right
    # EECS 281 Partition Body
    while True:
        # Find item to the left that is bigger than pivot
        while in_list[left] < in_list[pivot]:
            left += 1
        # Find item to the right that is smaller than pivot
        # Check left < right to not go out of bounds
        while (left < right and
               in_list[right - 1] >= in_list[pivot]):
            right -= 1

        if left >= right:
            break
        __swap_list_elts(in_list, left, right - 1)

    # Move pivot into correct position, return new index
    __swap_list_elts(in_list, left, pivot)
    # dprint(f"new_list: {in_list}")
    # dprint(f"split_left: {in_list[0:left]}")
    # dprint(f"split_right: {in_list[left+1:len(in_list)]}")
    # dprint(f"---\n")
    return left


def __quicksort_helper(in_list, left, right):
    if left + 1 >= right:
        # Arrays of size 1 are sorted
        return
    pivot = __partition(in_list, left, right)
    __quicksort_helper(in_list, left, pivot)
    __quicksort_helper(in_list, pivot + 1, right)


def quicksort(in_list):
    """Sort in_list using quicksort."""
    __quicksort_helper(in_list, 0, len(in_list))


if __name__ == '__main__':
    in_list = [0, 5, 6, 7, 3, 18, 2, 4, 9]
    iprint(f"Before: {in_list}")
    quicksort(in_list)
    assert check_sorted(in_list)
    iprint(f"After: {in_list}")
    # left = partition(in_list, 0, len(in_list))
