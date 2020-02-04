#!/usr/bin/env python
"""Time tester for sorts."""
import random
import sorts
import quicksort
from util import time_sort


def main():
    """Run main."""
    num_elts = 10**6
    orig = [random.randint(0, num_elts) for _ in range(num_elts)]
    elem_sorts = [sorts.insertion_sort, sorts.bubble_sort,
                  sorts.selection_sort]
    adv_sorts = [sorts.merge_sort, list.sort, quicksort.quicksort,
                 sorts.heap_sort, sorts.heap_sort2]
    bad_sorts = [sorts.bozo_sort, sorts.bogo_sort]
    holder = [elem_sorts, adv_sorts, bad_sorts]
    random.shuffle(holder[1])
    # XXX: Change me to be what sorts you want to run
    for sort in holder[1]:
        # Only merge sort impl is not in place
        time_sort(orig, sort, in_place=(sort != sorts.merge_sort))


if __name__ == '__main__':
    main()
