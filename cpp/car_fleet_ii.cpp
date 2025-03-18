/**
 * @file car_fleet_ii.cpp
 * @brief https://leetcode.com/problems/car-fleet-ii/
 * @date 2025-03-18
 */
#include <cassert>
#include <stack>
#include <vector>
using namespace std;

vector<double> getCollisionTimes(vector<vector<int>> const &cars) {
	stack<size_t> car_stack;
	vector<double> collision_times(cars.size(), -1);
	for (size_t i = cars.size(); i > 0; --i) {
		while (not car_stack.empty()) {
			if (cars[i - 1][1] <= cars[car_stack.top()][1]) {
				car_stack.pop();
				continue;
			}
			double time =
			    static_cast<double>(cars[car_stack.top()][0] - cars[i - 1][0]) /
			    (cars[i - 1][1] - cars[car_stack.top()][1]);
			if (collision_times[car_stack.top()] != -1 and
			    time > collision_times[car_stack.top()])
			{
				car_stack.pop();
			} else {
				collision_times[i - 1] = time;
				break;
			}
		}
		car_stack.push(i - 1);
	}
	return collision_times;
}

namespace {
	void compare_floats(
	    vector<double> const &expected,
	    vector<double> const &result
	) {
		for (size_t i = 0; i < expected.size(); ++i) {
			assert(abs(result[i] - expected[i]) < 1e-5);
		}
	}
} // namespace

int main() {
	compare_floats(
	    getCollisionTimes(
	        {{1, 5},
	         {6, 5},
	         {7, 5},
	         {14, 5},
	         {15, 3},
	         {16, 4},
	         {17, 5},
	         {18, 1},
	         {19, 2},
	         {20, 2}}
	    ),
	    vector(
	        {4.25000,
	         3.00000,
	         2.75000,
	         0.50000,
	         1.50000,
	         0.666667,
	         0.25000,
	         -1.00000,
	         -1.00000,
	         -1.00000}
	    )
	);
	compare_floats(
	    getCollisionTimes({{1, 2}, {2, 1}, {4, 3}, {7, 2}}),
	    vector({1.00000, -1.00000, 3.00000, -1.00000})
	);
	compare_floats(
	    getCollisionTimes({{3, 4}, {5, 4}, {6, 3}, {9, 1}}),
	    vector({2.00000, 1.00000, 1.50000, -1.00000})
	);
	return 0;
}
