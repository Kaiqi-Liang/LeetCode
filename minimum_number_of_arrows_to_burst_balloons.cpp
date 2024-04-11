/**
 * @file minimum_number_of_arrows_to_burst_balloons.cpp
 * @brief https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
 * @date 2024-04-11
 */
#include <algorithm>
#include <cassert>
#include <vector>
using namespace std;

int findMinArrowShots(vector<vector<int>> points) {
	sort(
	    points.begin(),
	    points.end(),
	    [](vector<int> const &point1, vector<int> const &point2) {
		    return point1[1] < point2[1];
	    }
	);
	size_t num_arrows = 1;
	int prev_end = points.front()[1];
	for (size_t i = 1; i < points.size(); ++i) {
		if (points[i][0] <= prev_end) continue;
		prev_end = points[i][1];
		++num_arrows;
	}
	return num_arrows;
}

int main() {
	assert(findMinArrowShots({{10, 16}, {2, 8}, {1, 6}, {7, 12}}) == 2);
	assert(findMinArrowShots({{1, 2}, {3, 4}, {5, 6}, {7, 8}}) == 4);
	assert(findMinArrowShots({{1, 2}, {2, 3}, {3, 4}, {4, 5}}) == 2);
	return 0;
}
