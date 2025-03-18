/**
 * @file longest_nice_subarray.cpp
 * @brief https://leetcode.com/problems/longest-nice-subarray/
 * @date 2025-03-18
 */
#include <cassert>
#include <vector>
using namespace std;

int longestNiceSubarray(vector<int> const &nums) {
	int left = 0;
	int longest_nice_subarray_size = 1;
	for (int right = 1; right < nums.size(); ++right) {
		int furthest_left_non_0_bitwise_and = left - 1;
		for (int i = left; i < right; ++i) {
			if (nums[i] & nums[right]) {
				furthest_left_non_0_bitwise_and = i;
			}
		}
		if (furthest_left_non_0_bitwise_and == left - 1) {
			longest_nice_subarray_size =
			    max(longest_nice_subarray_size, right - left + 1);
		} else {
			left = furthest_left_non_0_bitwise_and + 1;
		}
	}
	return longest_nice_subarray_size;
}

int main() {
	assert(longestNiceSubarray({1, 3, 8, 48, 10}) == 3);
	assert(longestNiceSubarray({3, 1, 5, 11, 13}) == 1);
	assert(
	    longestNiceSubarray(
	        {338970160,
	         525086042,
	         19212931,
	         213753017,
	         321613307,
	         553272419,
	         190837185,
	         548628106,
	         793546945,
	         243936947}
	    ) == 1
	);
}
