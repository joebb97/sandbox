"""Solve the bounded subset sum problem using bottom up DP."""


def subset_sum(items, target_sum):
    """You know the thing
        -- Joe Biden."""
    # NOTE to anyone playing along at home
    # [[0] * len(items)] * target_sum makes many rows
    # that are all a reference to the same list,
    # which is not normally intended.

    # answers[i][sum] is True when a subset of the sublist items[0:i]
    # has total sum equal to sum
    item_len = len(items)
    ans = [list(map(lambda x: x == 0, range(target_sum + 1)))
           for _ in range(item_len + 1)]

    for idx in range(1, item_len + 1):
        current_item = items[idx - 1]
        prev_ans = ans[idx - 1]
        next_ans = ans[idx]
        for current_sum in range(1, target_sum + 1):
            prev_sum = current_sum - current_item
            next_ans[current_sum] = ((prev_sum >= 0 and prev_ans[prev_sum])
                                     or prev_ans[current_sum])

    return ans[item_len][target_sum]


if __name__ == '__main__':
    items = [0, 5, 2, 1, 8, 3, 9, 4, 10, 12]
    # Get the target by adding 2 + 1 + 8 + 10
    target_sum = 17
    assert subset_sum(items, target_sum)
