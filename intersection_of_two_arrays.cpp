/**
 * @file intersection_of_two_arrays.cpp
 * @brief https://leetcode.com/problems/intersection-of-two-arrays/
 * @date 2024-03-10
 */
#include <algorithm>
#include <cassert>
#include <iterator>
#include <unordered_set>
#include <vector>
using namespace std;

vector<int> intersection(vector<int> nums1, vector<int> nums2) {
	unordered_set<int> res;
	sort(nums1.begin(), nums1.end());
	sort(nums2.begin(), nums2.end());
	set_intersection(
	    nums1.cbegin(),
	    nums1.cend(),
	    nums2.cbegin(),
	    nums2.cend(),
	    inserter(res, res.end())
	);
	return vector<int>(res.cbegin(), res.cend());
}

int main() {
	assert(intersection({1, 2, 2, 1}, {2, 2}) == vector<int>{2});
	assert(
	    intersection({4, 9, 5}, {9, 4, 9, 8, 4}) == vector<int>({4, 9}) or
	    intersection({4, 9, 5}, {9, 4, 9, 8, 4}) == vector<int>({9, 4})
	);
	return 0;
}
