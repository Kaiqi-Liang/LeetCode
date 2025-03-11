/**
 * @file max_num_of_k_sum_pairs.cpp
 * @brief https://leetcode.com/problems/max-number-of-k-sum-pairs/
 * @date 2025-03-11
 */
#include <cassert>
#include <vector>
using namespace std;

int maxOperations(vector<int> &nums, int k) {
	sort(nums.begin(), nums.end());
	size_t i = 0;
	size_t j = nums.size() - 1;
	int num_pairs = 0;
	while (i < j) {
		if (nums[i] + nums[j] > k) {
			--j;
		} else if (nums[i] + nums[j] < k) {
			++i;
		} else {
			++i;
			--j;
			++num_pairs;
		}
	}
	return num_pairs;
}

int main() {
	{
		vector<int> nums{1, 2, 3, 4};
		assert(maxOperations(nums, 5) == 2);
	}
	{
		vector<int> nums{3, 1, 3, 4, 3};
		assert(maxOperations(nums, 6) == 1);
	}
	return 0;
}
