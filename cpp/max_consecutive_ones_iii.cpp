/**
 * @file max_consecutive_ones.cpp
 * @brief https://leetcode.com/problems/max-consecutive-ones-iii/
 * @date 2025-03-11
 */
#include <cassert>
#include <vector>
using namespace std;

size_t longestOnes(vector<int> const &nums, size_t k) {
	size_t zero_count = 0;
	size_t left = 0;
	size_t longest_ones = 0;
	for (size_t right = 0; right < nums.size(); ++right) {
		if (nums[right] == 0) ++zero_count;
		while (zero_count > k) {
			if (nums[left] == 0) --zero_count;
			++left;
		}
		longest_ones = max(longest_ones, right - left + 1);
	}
	return longest_ones;
}

int main() {
	assert(longestOnes({1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0}, 2) == 6);
	assert(
	    longestOnes(
	        {0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1},
	        3
	    ) == 10
	);
	return 0;
}
