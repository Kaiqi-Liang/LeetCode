/**
 * @file merge_two_2d_arrays_by_summing_values.cpp
 * @brief https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/
 * @date 2025-03-02
 */
#include <cassert>
#include <vector>
using namespace std;

vector<vector<int>> mergeArrays(
    vector<vector<int>> const &nums1,
    vector<vector<int>> const &nums2
) {
	std::vector<int> sums(1000, 0);
	int i = 0, j = 0;
	while (i < nums1.size() and j < nums2.size()) {
		if (nums1[i][0] < nums2[j][0]) {
			sums[nums1[i][0] - 1] += nums1[i][1];
			++i;
		} else if (nums1[i][0] > nums2[j][0]) {
			sums[nums2[j][0] - 1] += nums2[j][1];
			++j;
		} else {
			sums[nums2[j][0] - 1] += nums2[j][1] + nums1[i][1];
			++i;
			++j;
		}
	}
	while (i < nums1.size()) {
		sums[nums1[i][0] - 1] += nums1[i][1];
		++i;
	}
	while (j < nums2.size()) {
		sums[nums2[j][0] - 1] += nums2[j][1];
		++j;
	}
	vector<vector<int>> res;
	for (int k = 0; k < 1000; ++k) {
		if (sums[k] > 0) {
			res.push_back({k + 1, sums[k]});
		}
	}
	return res;
}

int main() {
	assert(
	    mergeArrays({{1, 2}, {2, 3}, {4, 5}}, {{1, 4}, {3, 2}, {4, 1}}) ==
	    vector({vector({1, 6}), vector({2, 3}), vector({3, 2}), vector({4, 6})})
	);
	assert(
	    mergeArrays({{2, 4}, {3, 6}, {5, 5}}, {{1, 3}, {4, 3}}) ==
	    vector(
	        {vector({1, 3}),
	         vector({2, 4}),
	         vector({3, 6}),
	         vector({4, 3}),
	         vector({5, 5})}
	    )
	);
	return 0;
}
