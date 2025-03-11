/**
 * @file move_zeros.cpp
 * @brief https://leetcode.com/problems/move-zeroes/
 * @date 2025-03-11
 */
#include <cassert>
#include <vector>
using namespace std;

void moveZeroes(vector<int> &nums) {
	vector<size_t> num_zeros_in_front(nums.size(), 0);
	for (size_t i = 1; i < nums.size(); ++i) {
		num_zeros_in_front[i] = num_zeros_in_front[i - 1];
		if (nums[i - 1] == 0) ++num_zeros_in_front[i];
	}
	for (size_t i = 1; i < nums.size(); ++i) {
		nums[i - num_zeros_in_front[i]] = nums[i];
	}
	fill(nums.end() - num_zeros_in_front[nums.size() - 1], nums.end(), 0);
}

int main() {
	vector<int> nums{0, 0, 1, -2, 0, 4, 0, 5, 5, 5, 0};
	moveZeroes(nums);
	assert(nums == vector({1, -2, 4, 5, 5, 5, 0, 0, 0, 0, 0}));
	return 0;
}
