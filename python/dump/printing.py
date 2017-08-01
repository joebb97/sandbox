#!/usr/bin/env python
def slicing_test():
    string = "   -100.33,22.1,0 -88,17.5,0"
    lol = string.strip().split()
    for part in lol:
        if part != ' ':
            print part.split(',')

if __name__ == '__main__':
    slicing_test()
    
