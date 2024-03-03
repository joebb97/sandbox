"""Indent some stuff."""
import sys

if __name__ == "__main__":
    fname = sys.argv[1]
    with open(fname) as f:
        d = f.read()
        lines = d.split('\n')
