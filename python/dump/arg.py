#!/usr/bin/env python
import argparse

def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("-n", type=int, default=3)
    parser.add_argument("-p", action="store_true")
    parser.add_argument("-i", "--ips", nargs="+", type=str,
                        default=["10.0.3.154"])
    args = parser.parse_args()
    return args

def args_join_test():
    args = get_args()
    print args.ips
    for ip in args.ips:
        print ip
    if args.p:
        str_list = []
        for i in range(args.n):
            str_list.append("line {}".format(i))
        print "\n".join(str_list)

if __name__ == '__main__':
    args_join_test()
