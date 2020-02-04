"""My bad implementation of some sorting algorithms."""
import heapq
import random
from util import check_sorted


def heap_sort(alist):
    """Heap sort using heapify."""
    temp = []
    heapq.heapify(alist)
    while alist:
        temp.append(heapq.heappop(alist))
    alist[:] = temp


def heap_sort2(alist):
    """Heap sort without heapify."""
    temp = []
    for item in alist:
        heapq.heappush(temp, item)

    alist[:] = [heapq.heappop(temp) for i in range(len(temp))]


def merge(right, left):
    """Merge sort (probably copy pasted, don't remember)."""
    ret = []
    right_pos = 0
    left_pos = 0
    while right_pos < len(right) and left_pos < len(left):
        if right[right_pos] <= left[left_pos]:
            ret.append(right[right_pos])
            right_pos += 1
        else:
            ret.append(left[left_pos])
            left_pos += 1

    while right_pos < len(right):
        ret.append(right[right_pos])
        right_pos += 1

    while left_pos < len(left):
        ret.append(left[left_pos])
        left_pos += 1

    return ret


def merge_sort(alist):
    """Merge sort implementation (copy pasted I believe)."""
    if len(alist) == 1:
        return alist
    mid = len(alist) // 2
    right = alist[mid:]
    left = alist[:mid]
    left = merge_sort(left)
    right = merge_sort(right)
    return merge(left, right)


def merge_sort2(alist):
    """Merge sort implementation (copy pasted I believe)."""
    if len(alist) > 1:
        mid = len(alist)//2
        lefthalf = alist[:mid]
        righthalf = alist[mid:]

        merge_sort2(lefthalf)
        merge_sort2(righthalf)

        i = 0
        j = 0
        k = 0
        while i < len(lefthalf) and j < len(righthalf):
            if lefthalf[i] < righthalf[j]:
                alist[k] = lefthalf[i]
                i = i+1
            else:
                alist[k] = righthalf[j]
                j = j+1
            k = k+1

        while i < len(lefthalf):
            alist[k] = lefthalf[i]


def bozo_sort(alist):
    """Hilariously bad bozo_sort."""
    while not check_sorted(alist):
        pos_one = random.randint(0, len(alist) - 1)
        pos_two = random.randint(0, len(alist) - 1)
        while pos_one == pos_two:
            pos_one = random.randint(0, len(alist) - 1)
        alist[pos_one], alist[pos_two] = alist[pos_two], alist[pos_one]


def bogo_sort(alist):
    while not check_sorted(alist):
        random.shuffle(alist)


def bubble_sort(alist):
    for i in range(0, len(alist)):
        swapped = False
        for j in range(len(alist) - 1, i, -1):
            if alist[j - 1] > alist[j]:
                swapped = True
                alist[j - 1], alist[j] = alist[j], alist[j - 1]
        if not swapped:
            break


def insertion_sort(alist):
    for i in range(1, len(alist)):
        for j in range(i, 0, -1):
            if alist[j] < alist[j - 1]:
                alist[j - 1], alist[j] = alist[j], alist[j - 1]


def selection_sort(alist):
    for i in range(0, len(alist)):
        smallest_pos = i
        for j in range(i, len(alist)):
            if alist[j] < alist[smallest_pos]:
                smallest_pos = j
        alist[i], alist[smallest_pos] = alist[smallest_pos], alist[i]
