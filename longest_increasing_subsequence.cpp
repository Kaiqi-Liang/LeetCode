/**
 * @file longest_increasing_subsequence.cpp
 * @brief https://leetcode.com/problems/longest-increasing-subsequence/
 * @date 2021-10-22
 */
#include <algorithm>
#include <iostream>
#include <vector>

auto lengthOfLIS(std::vector<std::size_t> nums) -> std::size_t {
	auto opt = std::vector<std::size_t>(nums.size(), 1);
	for (auto i = std::size_t{1}; i < nums.size(); ++i) {
		auto prev = std::vector<std::size_t>{};
		for (auto j = std::size_t{0}; j < i; ++j) {
			if (nums[j] < nums[i]) {
				prev.push_back(opt[j]);
			}
		}
		auto max = std::max_element(prev.cbegin(), prev.cend());
		if (max != prev.cend()) opt[i] = *max + 1;
	}
	return *std::max_element(opt.cbegin(), opt.cend());
}

auto main() -> int {
	assert(lengthOfLIS({10,9,2,5,3,7,101,18}) == 4);
	assert(lengthOfLIS({0,1,0,3,2,3}) == 4);
	assert(lengthOfLIS({7,7,7,7,7,7,7}) == 1);
	assert(lengthOfLIS({1,3,4,5,6,2,8,7}) == 6);
	return 0;
}
