/**
 * @file divide_array_into_equal_pairs.cpp
 * @brief https://leetcode.com/problems/divide-array-into-equal-pairs/
 * @date 2025-03-17
 */
#include <algorithm>
#include <cassert>
#include <unordered_map>
#include <vector>
using namespace std;

bool divideArray(vector<int> const &nums) {
	unordered_map<int, size_t> counter;
	for (int num : nums) {
		++counter[num];
	}
	return find_if(
	           counter.cbegin(),
	           counter.cend(),
	           [](pair<int, size_t> const &count) {
		           return count.second % 2 != 0;
	           }
	       ) == counter.cend();
}

int main() {
	assert(divideArray({3, 2, 3, 2, 2, 2}));
	assert(not divideArray({1, 2, 3, 4}));
	return 0;
}
