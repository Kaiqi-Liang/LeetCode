/**
 * @file longest_well_performing_interval.cpp
 * @brief https://leetcode.com/problems/longest-well-performing-interval/
 * @date 2025-03-19
 */
#include <algorithm>
#include <cassert>
#include <unordered_map>
#include <vector>
using namespace std;

int longestWPI(vector<int> const &hours) {
	vector<int> tiring_days;
	transform(
	    hours.cbegin(),
	    hours.cend(),
	    back_inserter(tiring_days),
	    [](int hour) { return hour > 8 ? 1 : -1; }
	);

	vector<int> prefix_sum(tiring_days.size(), tiring_days.front());
	for (size_t i = 1; i < tiring_days.size(); ++i) {
		prefix_sum[i] = tiring_days[i] + prefix_sum[i - 1];
	}

	size_t longest_wpi = 0;
	unordered_map<int, size_t> earliest_prefix_sum_index;
	for (size_t i = 0; i < prefix_sum.size(); ++i) {
		if (not earliest_prefix_sum_index.contains(prefix_sum[i])) {
			earliest_prefix_sum_index[prefix_sum[i]] = i;
		}
		if (prefix_sum[i] > 0) {
			longest_wpi = i + 1;
		} else {
			int valid_prefix_sum = prefix_sum[i] - 1;
			if (earliest_prefix_sum_index.contains(valid_prefix_sum)) {
				longest_wpi =
				    max(longest_wpi,
				        i - earliest_prefix_sum_index[valid_prefix_sum]);
			}
		}
	}
	return static_cast<int>(longest_wpi);
}

int main() {
	assert(longestWPI({6, 9}) == 1);
	assert(longestWPI({9, 6, 6, 9, 9, 9, 6, 6}) == 7);
	assert(longestWPI({9, 9, 6, 0, 6, 6, 9}) == 3);
	assert(longestWPI({6, 9, 6, 6, 6, 9, 6, 9, 9}) == 5);
	assert(longestWPI({6, 6, 9}) == 1);
	assert(longestWPI({9, 6, 9}) == 3);
	assert(longestWPI({9, 6, 6}) == 1);
	assert(longestWPI({6, 6, 6}) == 0);
	assert(longestWPI({6, 9, 9}) == 3);
	assert(longestWPI({9, 6, 6, 9, 9}) == 5);
	assert(longestWPI({6}) == 0);
	assert(longestWPI({10}) == 1);
	return 0;
}
