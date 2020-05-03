"""Solve the 0-1 knapsack problem using bottom up dynamic programming."""
import random
from collections import namedtuple

Item = namedtuple('Item', ['value', 'weight'])


def knapsack_bottom_up(items, max_weight):
    """Do the thing."""
    # answers[weight][idx] contains the best yield for a knapsack
    # of size weight containing all items before idx
    answers = [[0] * (max_weight + 1) for _ in range(len(items) + 1)]
    item_len = len(items)
    space_used = 0
    for idx in range(1, item_len + 1):
        current_item = items[idx - 1]
        prev_answer = answers[idx - 1]
        next_answer = answers[idx]
        for weight in range(1, max_weight + 1):
            space_used = weight - current_item.weight
            if space_used >= 0:
                next_answer[weight] = max(
                    prev_answer[weight],
                    current_item.value + prev_answer[space_used]
                )
            else:
                next_answer[weight] = prev_answer[weight]

    return answers[item_len][max_weight], space_used


if __name__ == '__main__':
    items = [Item(2, 2), Item(3, 10), Item(4, 2),
             Item(15, 1), Item(1, 3), Item(20, 21)]
    # Order shouldn't affect correctness
    random.shuffle(items)
    best, space_used = knapsack_bottom_up(items, max_weight=20)
    # Correct answer is 25, which will use 18 weight
    print(best, space_used)
    assert best == 25

    items = [Item(60, 10), Item(100, 20), Item(120, 30)]
    best, space_used = knapsack_bottom_up(items, max_weight=50)
    # Correct answer is 220, which will use 50 weight
    print(best, space_used)
    assert best == 220
