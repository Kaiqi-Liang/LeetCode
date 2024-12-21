/**
 * @file merge_sorted_arrays.cpp
 * @brief https://leetcode.com/problems/merge-sorted-array/
 * @date 2024-12-21
 */
#include <cassert>
#include <vector>
using namespace std;
void merge_sorted_array(
    vector<int> &nums1,
    int m,
    vector<int> const &nums2,
    int n
) {
	int i = m - 1;
	int j = n - 1;
	int k = nums1.size() - 1;
	// compare nums1 and nums2
	while (i >= 0 and j >= 0) {
		if (nums1[i] >= nums2[j]) {
			nums1[k] = nums1[i];
			--i;
		} else {
			nums1[k] = nums2[j];
			--j;
		}
		--k;
	}
	// go through the rest of nums2
	while (j >= 0) {
		nums1[k] = nums2[j];
		--k;
		--j;
	}
}

int main() {
	{
		auto nums = vector<int>{1, 2, 3, 0, 0, 0};
		merge_sorted_array(nums, 3, vector<int>({2, 5, 6}), 3);
		assert(nums == vector<int>({1, 2, 2, 3, 5, 6}));
	}
	{
		auto nums = vector<int>{1};
		merge_sorted_array(nums, 1, {}, 0);
		assert(nums == vector<int>({1}));
	}
	{
		auto nums = vector<int>{0};
		merge_sorted_array(nums, 0, {1}, 1);
		assert(nums == vector<int>({1}));
	}
	return 0;
}
