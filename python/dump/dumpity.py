# Given an int array, return the same array with different orders so that no same value is adjacent to each other
# [1, 1, 2] -> [1, 2, 1]
# [1, 2, 2] -> [2, 1, 2]
# [1, 1, 1, 1, 1] -> invalid input return []
# [1, 1, 1, 2] ? invalid
# [1, 1, 1, 1, 2, 2] ? invalid
# [1, 1, 1, 2, 2] ? [1, 2, 1, 2, 1] valid

# num_1 - 1 <= num_non_1 -> valid
# otherwise -> invalid

from collections import OrderedDict
import pdb

def sort_by_values(mydict):
    # will figure out the implementation later
    return mydict


# TODO: REIMPLEMENT WITH PRIORITY QUEUE
def no_adjacent(arr):
    # return an array with values adjacent to one another having the same value
    counts = {}
    for item in arr:
        if item not in counts:
            counts[item] = 1
        else:
            counts[item] += 1

    ret = []
    # TODO, add invalid check
    new_dict = OrderedDict(sorted(counts.items(), key=lambda item: item[1], reverse=True))
    while len(ret) != len(arr):
        for key, value in new_dict.items():
            if value != 0:
                ret.append(key)
                new_dict[key] -= 1

    return ret


if __name__ == '__main__':
    print(no_adjacent([1, 1, 2]))
    print(no_adjacent([1, 2, 2]))
    print(no_adjacent([1, 1, 1, 2, 2]))
    print(no_adjacent([1, 1, 1, 1, 2, 2]))
