/**
 * @file largest_rectangle_in_histogram.cpp
 * @brief https://leetcode.com/problems/largest-rectangle-in-histogram/
 * @date 2024-04-30
 */
#include <cassert>
#include <stack>
#include <utility>
#include <vector>
using namespace std;

size_t largestRectangleArea(vector<size_t> const &heights) {
	stack<pair<size_t, size_t>> monotonic;
	size_t max_area = 0;
	for (size_t i = 0; i < heights.size(); ++i) {
		size_t width = 0;
		while (not monotonic.empty() and monotonic.top().second > heights[i]) {
			auto [start, height] = monotonic.top();
			width = i - start;
			max_area = max(max_area, width * height);
			monotonic.pop();
		}
		monotonic.emplace(i - width, heights[i]);
	}
	while (not monotonic.empty()) {
		auto [start, height] = monotonic.top();
		max_area = max(max_area, (heights.size() - start) * height);
		monotonic.pop();
	}
	return max_area;
}

int main() {
	assert(largestRectangleArea({3, 6, 5, 7, 4, 8, 1, 0}) == 20);
	assert(largestRectangleArea({2, 1, 5, 6, 2, 3}) == 10);
	assert(largestRectangleArea({2, 4}) == 4);
	return 0;
}
