/**
 * @file find_minimum_in_rotated_sorted_array.cpp
 * @brief https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
 * @date 2025-03-12
 */
#include <cassert>
#include <vector>
using namespace std;

int findMin(vector<int> const &nums) {
	int low = 0;
	int high = nums.size();
	int smallest;
	while (low <= high) {
		int mid = (low + high) / 2;
		if (nums[mid] > nums.back()) {
			low = mid + 1;
		} else {
			high = mid - 1;
			smallest = nums[mid];
		}
	}
	return smallest;
}

int main() {
	assert(findMin({3, 4, 5, 1, 2}) == 1);
	assert(findMin({4, 5, 6, 7, 0, 1, 2}) == 0);
	assert(findMin({11, 13, 15, 17}) == 11);
	return 0;
}
