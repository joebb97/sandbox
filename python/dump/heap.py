#!/usr/bin/env python
import heapq

def heap_sort(alist):
    temp = []
    heapq.heapify(alist)
    while alist:
        temp.append(heapq.heappop(alist))
    alist[:] = temp


def test_heapify(alist):
    temp = alist
    print 'temp before: ', temp
    print 'alist before: ', alist
    heapq.heapify(temp)
    print 'temp after: ', temp
    print 'alist after: ', alist

# test_heapify(alist)
# print alist
alist = [3, 8, 10000, 8989897, 2 ]
heap_sort(alist)
print alist
