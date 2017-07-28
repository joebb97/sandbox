#!/usr/bin/env python
import argparse

def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("-n", type=int, default=3)
    parser.add_argument("-p", action="store_true")
    args = parser.parse_args()
    return args

def args_join_test():
    args = get_args()
    if args.p:
        str_list = []
        for i in range(args.n):
            str_list.append("line {}".format(i))
        print "\n".join(str_list)

def slicing_test():
    string = "   -100.33,22.1,0 -88,17.5,0"
    lol = string.strip().split()
    for part in lol:
        if part != ' ':
            print part.split(',')

if __name__ == '__main__':
    slicing_test()
    
