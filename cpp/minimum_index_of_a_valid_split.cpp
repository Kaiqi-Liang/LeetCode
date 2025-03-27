/**
 * @file minimum_index_of_a_valid_split.cpp
 * @brief https://leetcode.com/problems/minimum-index-of-a-valid-split/
 * @date 2025-03-27
 */
#include <algorithm>
#include <cassert>
#include <unordered_map>
#include <vector>
using namespace std;

using FrequencyMap = unordered_map<int, size_t>;
int minimumIndex(vector<int> const &nums) {
	FrequencyMap count =
	    ranges::fold_left(nums, FrequencyMap{}, [](FrequencyMap a, int c) {
		    ++a[c];
		    return a;
	    });
	auto [dominant_element, num_dominant_elements] = *ranges::max_element(
	    count,
	    [](pair<int, size_t> const &a, pair<int, size_t> const &b) {
		    return a.second < b.second;
	    }
	);
	size_t num_dominant_left = 0;
	for (size_t i = 0; i < nums.size(); ++i) {
		if (nums[i] == dominant_element) {
			++num_dominant_left;
			size_t split = i + 1;
			if (num_dominant_left > split / 2 and
			    (num_dominant_elements - num_dominant_left) >
			        (nums.size() - split) / 2)
			{
				return i;
			}
		}
	}
	return -1;
}

int main() {
	assert(minimumIndex({1, 2, 2, 2}) == 2);
	assert(minimumIndex({2, 1, 3, 1, 1, 1, 7, 1, 2, 1}) == 4);
	assert(minimumIndex({3, 3, 3, 3, 7, 2, 2}) == -1);
	assert(minimumIndex({2, 1, 2, 2}) == 0);
	return 0;
}
