"""Utilities used by sorts and main."""
import time


def check_sorted(alist):
    """Check if the list is actually sorted."""
    for i in range(0, len(alist) - 1):
        if alist[i] > alist[i + 1]:
            return False
    return True


def time_sort(orig, sort, in_place):
    """Run a sorting algorithm, then time it to see how long it took."""
    alist = list(orig)
    start = time.time()
    if in_place:
        sort(alist)
    else:
        # Merge sort impl is not in place
        alist = sort(alist)
    end = time.time()
    if not check_sorted(alist):
        print(str(sort) + "failed!!!")
        return
    print(str(sort) + " took: " + str(end - start) + " seconds")
