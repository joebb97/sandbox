#!/usr/bin/env python3

def has_target_window(arr: [int], target: int) -> (int, int, bool):
    """
        arr: integers are greater than 0
        target: greater than 0
        return: (left side of window, right side of window, target found)
    """
    left, right, wsum = -1, -1, 0
    arr_len = len(arr)
    while wsum != target:
        if wsum < target:
            right += 1
            if right >= arr_len:
                break
            wsum += arr[right]
        elif wsum > target:
            left += 1
            if left >= arr_len:
                break
            wsum -= arr[left]
        print(locals())
    return (left, right, wsum == target)


if __name__ == "__main__":
    # print(has_target_window([1, 3, 8, 10], 18))
    print(has_target_window([10], 10))
    print(has_target_window([0, 10], 10))
