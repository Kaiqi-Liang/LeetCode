/**
 * @file maximum_average_subarray_i.cpp
 * @brief https://leetcode.com/problems/maximum-average-subarray-i/
 * @date 2025-03-11
 */
#include <cassert>
#include <climits>
#include <vector>
using namespace std;

double findMaxAverage(vector<int> const &nums, size_t k) {
	long double running_sum = 0;
	long double max_sum = LONG_MIN;
	for (size_t i = 0; i < nums.size(); ++i) {
		if (i == k) max_sum = running_sum;
		running_sum += nums[i];
		if (i >= k) {
			running_sum -= nums[i - k];
			max_sum = max(max_sum, running_sum);
		}
	}
	return max(max_sum, running_sum) / k;
}

int main() {
	assert(findMaxAverage({1, 12, -5, -6, 50, 3}, 4) == 12.75000);
	assert(findMaxAverage({5}, 1) == 5.000);
	assert(findMaxAverage({-1}, 1) == -1);
	assert(
	    abs(findMaxAverage({9, 7, 3, 5, 6, 2, 0, 8, 1, 9}, 6) - 5.33333) < 1e-5
	);
	assert(
	    abs(findMaxAverage(
	            {-6662, 5432,  -8558, -8935, 8731,  -3083, 4115,  9931,  -4006,
	             -3284, -3024, 1714,  -2825, -2374, -2750, -959,  6516,  9356,
	             8040,  -2169, -9490, -3068, 6299,  7823,  -9767, 5751,  -7897,
	             6680,  -1293, -3486, -6785, 6337,  -9158, -4183, 6240,  -2846,
	             -2588, -5458, -9576, -1501, -908,  -5477, 7596,  -8863, -4088,
	             7922,  8231,  -4928, 7636,  -3994, -243,  -1327, 8425,  -3468,
	             -4218, -364,  4257,  5690,  1035,  6217,  8880,  4127,  -6299,
	             -1831, 2854,  -4498, -6983, -677,  2216,  -1938, 3348,  4099,
	             3591,  9076,  942,   4571,  -4200, 7271,  -6920, -1886, 662,
	             7844,  3658,  -6562, -2106, -296,  -3280, 8909,  -8352, -9413,
	             3513,  1352,  -8825},
	            90
	        ) -
	        37.25556) < 1e-5
	);
	return 0;
}
