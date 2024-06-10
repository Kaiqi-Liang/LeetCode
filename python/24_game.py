"""
https://leetcode.com/problems/24-game/
"""
from itertools import combinations
from operator import add, sub, mul, truediv


def judgePoint24(cards: list[int]) -> bool:
    if len(cards) == 1:
        return abs(cards[0] - 24) < 10e-10
    for i, j in combinations(range(len(cards)), 2):
        for operator in [add, sub, mul, truediv]:
            remaining_indices = list(range(len(cards)))
            remaining_indices.remove(i)
            remaining_indices.remove(j)
            remaining_numbers = [cards[index] for index in remaining_indices]
            try:
                if judgePoint24(
                    [*remaining_numbers, operator(cards[i], cards[j])]
                ) or judgePoint24([*remaining_numbers, operator(cards[j], cards[i])]):
                    return True
            except ZeroDivisionError:
                continue
    return False


if __name__ == "__main__":
    assert judgePoint24([3, 3, 8, 8])
    assert judgePoint24([7, 2, 6, 6])
    assert judgePoint24([1, 3, 4, 6])
    assert judgePoint24([1, 3, 4, 6])
    assert judgePoint24([0, 0, 0, 24])
    assert not judgePoint24([0, 0, 0, 0])
    assert judgePoint24([4, 1, 8, 7])
    assert not judgePoint24([1, 1, 1, 1])
