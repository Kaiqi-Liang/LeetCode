/**
 * @file remove_duplicates_from_sorted_array_ii.cpp
 * @brief https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
 * @date 2024-04-08
 */
#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

int removeDuplicates(vector<int> &nums) {
	unordered_map<int, size_t> frequency;
	int count = nums.size();
	for (size_t i = 0; i < count;) {
		if (frequency[nums[i]] == 2) {
			for (size_t j = i; j < nums.size() - 1; ++j) {
				nums[j] = nums[j + 1];
			}
			--count;
		} else {
			++frequency[nums[i]];
			++i;
		}
	}
	return count;
}

int main() {
	vector<int> nums = {1, 1, 1, 2, 2, 3};
	vector<int> expectedNums =
	    {1, 1, 2, 2, 3};
	int k = removeDuplicates(nums);
	assert(k == expectedNums.size());
	for (int i = 0; i < k; i++) {
		assert(nums[i] == expectedNums[i]);
	}
	return 0;
}
