/**
 * @file daily_temperatures.cpp
 * @brief https://leetcode.com/problems/daily-temperatures/
 * @date 2024-11-30
 *
 */
#include <cassert>
#include <queue>
#include <utility>
#include <vector>
using namespace std;
vector<int> dailyTemperatures(vector<int> const &temperatures) {
	using Element = pair<int, size_t>;
	priority_queue<Element, vector<Element>, greater<Element>> heap;
	vector<int> res(temperatures.size(), 0);
	for (size_t i = 0; i < temperatures.size(); ++i) {
		while (not heap.empty() and heap.top().first < temperatures[i]) {
			size_t j = heap.top().second;
			res[j] = i - j;
			heap.pop();
		}
		heap.emplace(temperatures[i], i);
	}
	return res;
}

int main() {
	assert(
	    dailyTemperatures({73, 74, 75, 71, 69, 72, 76, 73}) ==
	    vector<int>({1, 1, 4, 2, 1, 1, 0, 0})
	);
	assert(dailyTemperatures({30, 40, 50, 60}) == vector<int>({1, 1, 1, 0}));
	assert(dailyTemperatures({30, 60, 90}) == vector<int>({1, 1, 0}));
	return 0;
}
