import sys
import random
import time

def merge(right, left):
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
    if len(alist) == 1:
        return alist
    mid = len(alist) // 2
    right = alist[mid:]
    left = alist[:mid]
    left = merge_sort(left)
    right = merge_sort(right)
    return merge(left, right)


def bozo_sort(alist):
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

def check_sorted(alist):
    for i in range(0, len(alist) - 1):
        if alist[i] > alist[i + 1]:
            return False
    return True

def time_sort(orig, sort):
    alist = list(orig)
    start = time.time()
    if sort == merge_sort:
        alist = sort(alist)
    else:
        sort(alist)
    end = time.time()
    if not check_sorted(alist):
        print str(sort) + "failed!!!"
        return
    print str(sort) + " took: ", str(end - start) + " seconds"

def main():
    orig = []
    for i in range(11):
        orig.append(random.randint(0, 1000))
    sorts = [list.sort, merge_sort, bubble_sort,
             selection_sort, insertion_sort]
    bad_sorts = [bozo_sort, bogo_sort]
    random.shuffle(sorts)
    random.shuffle(bad_sorts)
    for sort in sorts:
        time_sort(orig, sort)
    for bad_sort in bad_sorts:
        time_sort(orig, bad_sort)



if __name__ == '__main__':
    main()