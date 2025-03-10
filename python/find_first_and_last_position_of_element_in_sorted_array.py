"""
https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
"""
from typing import List


def searchRange(nums: List[int], target: int) -> List[int]:
    if len(nums) == 0:
        return [-1, -1]
    found = False

    # binary search for the biggest value smaller than the target
    low = 0
    high = len(nums) - 1
    start = -1
    while low <= high:
        mid = (low + high) // 2
        if nums[mid] < target:
            start = mid
            low = mid + 1
        else:
            if nums[mid] == target:
                found = True
            high = mid - 1
    start = max(start + 1, 0)

    # binary search for the smallest value bigger than the target
    low = 0
    high = len(nums) - 1
    end = -1
    while low <= high:
        mid = (low + high) // 2
        if nums[mid] <= target:
            low = mid + 1
        else:
            end = mid
            high = mid - 1
    if end == -1:
        end = len(nums) - 1
    else:
        end -= 1

    if not found:
        return [-1, -1]
    return [start, end]


if __name__ == "__main__":
    assert searchRange([5, 7, 7, 8, 8, 10], 8) == [3, 4]
    assert searchRange([5, 7, 7, 8, 8, 10], 6) == [-1, -1]
    assert searchRange([], 0) == [-1, -1]
