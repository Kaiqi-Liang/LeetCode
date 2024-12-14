"""
https://leetcode.com/problems/can-place-flowers/
"""

from typing import List


def can_place_flowers(flowerbeds: List[int], n: int) -> bool:
    """
    :type flowerbed: List[int]
    :type n: int
    :rtype: bool
    """
    if flowerbeds[0] == 0:
        flowerbeds = [1, 0] + flowerbeds
    if flowerbeds[-1] == 0:
        flowerbeds += [0, 1]
    num_zeros = 0
    for i in range(1, len(flowerbeds)):
        if flowerbeds[i] == 0:
            num_zeros += 1
        else:
            n -= (num_zeros - 1) / 2
            num_zeros = 0
    return n <= 0


if __name__ == "__main__":
    assert can_place_flowers([1, 0, 0, 0, 1], 1)
    assert not can_place_flowers([1, 0, 0, 0, 1], 2)
