/**
 * @file find_first_and_last_position_of_element_in_sorted_array.cpp
 * @brief
 * https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
 * @date 2025-03-11
 */
#include <algorithm>
#include <cassert>
#include <iterator>
#include <vector>
using namespace std;

vector<int> searchRange(vector<int> const &nums, int target) {
	auto left = lower_bound(nums.cbegin(), nums.cend(), target);
	auto right = upper_bound(nums.cbegin(), nums.cend(), target);
	return left == right
	           ? vector{-1, -1}
	           : vector{
	                 static_cast<int>(distance(nums.cbegin(), left)),
	                 static_cast<int>(distance(nums.cbegin(), --right)),
	             };
}

int main() {
	assert(searchRange({5, 7, 7, 8, 8, 10}, 8) == vector({3, 4}));
	assert(searchRange({5, 7, 7, 8, 8, 10}, 6) == vector({-1, -1}));
	assert(searchRange({1, 1, 1}, 1) == vector({0, 2}));
	assert(searchRange({}, 0) == vector({-1, -1}));
	return 0;
}
