#!/usr/bin/env python

def main(list1, list2):
    print 'BEGIN DURING'
    list1.append(7)
    print list1
    print list2
    print 'END DURING'


if __name__ == '__main__':
    list1 = [1, 3 ,5]
    list2 = list1
    print 'BEFORE'
    print list1
    print list2
    main(list1,list2)
    print 'AFTER'
    print list1
    print list2
