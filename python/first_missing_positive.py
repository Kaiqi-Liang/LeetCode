"""
https://leetcode.com/problems/first-missing-positive/
"""


def first_missing_positive(nums: list[int]) -> int:
    """
    Given an unsorted integer array nums, find the smallest missing positive integer.
    """
    nums = sorted(set(filter(lambda x: x > 0, nums)))
    curr = 1
    for num in nums:
        if num > curr:
            return curr
        curr += 1
    return curr


if __name__ == "__main__":
    assert first_missing_positive([7, 8, 9, 11, 12]) == 1
    assert first_missing_positive([1, 2, 0]) == 3
    assert first_missing_positive([3, 4, -1, 1]) == 2
    assert first_missing_positive([1, 1, 1]) == 2
