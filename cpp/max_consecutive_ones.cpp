/**
 * @file max_consecutive_ones.cpp
 * @brief https://leetcode.com/problems/max-consecutive-ones/
 * @date 2025-03-11
 */
#include <cassert>
#include <vector>
using namespace std;

int findMaxConsecutiveOnes(vector<int> const &nums) {
	int max_consecutive_ones = 0;
	int running_ones = 0;
	for (int num : nums) {
		if (num == 0)
			running_ones = 0;
		else
			++running_ones;
		max_consecutive_ones = max(max_consecutive_ones, running_ones);
	}
	return max_consecutive_ones;
}

int main() {
	assert(findMaxConsecutiveOnes({1, 1, 0, 1, 1, 1}) == 3);
	assert(findMaxConsecutiveOnes({1, 0, 1, 1, 0, 1}) == 2);
	return 0;
}
