/**
 * @file number_of_ways_to_arrive_at_destination.cpp
 * @brief https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/
 * @date 2025-03-23
 */
#include <cassert>
#include <queue>
#include <utility>
#include <vector>
using namespace std;

constexpr long long MOD = 1e9 + 7;
using Neighbour = pair<long long, int>;

int countPaths(int n, vector<vector<int>> const &roads) {
	vector<vector<Neighbour>> graph(n);
	for (vector<int> const &road : roads) {
		graph[road[0]].emplace_back(road[2], road[1]);
		graph[road[1]].emplace_back(road[2], road[0]);
	}

	vector<int> num_paths(n);
	num_paths[0] = 1;
	priority_queue<Neighbour, vector<Neighbour>, greater<Neighbour>> min_heap;
	min_heap.emplace(0, 0);
	vector<long long> shortest_times(n, LLONG_MAX);
	while (not min_heap.empty()) {
		auto [curr_time, curr_node] = min_heap.top();
		min_heap.pop();
		if (curr_time > shortest_times[curr_node]) continue;
		for (auto &&[time, neighbour] : graph[curr_node]) {
			long long total_time = time + curr_time;
			if (shortest_times[neighbour] == total_time) {
				num_paths[neighbour] =
				    (num_paths[neighbour] + num_paths[curr_node]) % MOD;
			} else if (shortest_times[neighbour] > total_time) {
				shortest_times[neighbour] = total_time;
				num_paths[neighbour] = num_paths[curr_node];
				min_heap.emplace(total_time, neighbour);
			}
		}
	}
	return num_paths.back();
}

int main() {
	assert(
	    countPaths(
	        7,
	        {{0, 6, 7},
	         {0, 1, 2},
	         {1, 2, 3},
	         {1, 3, 3},
	         {6, 3, 3},
	         {3, 5, 1},
	         {6, 5, 1},
	         {2, 5, 1},
	         {0, 4, 5},
	         {4, 6, 2}}
	    ) == 4
	);
	assert(countPaths(2, {{1, 0, 10}}) == 1);
	assert(
	    countPaths(
	        12,
	        {{1, 0, 2348},   {2, 1, 2852},   {2, 0, 5200},   {3, 1, 12480},
	         {2, 3, 9628},   {4, 3, 7367},   {4, 0, 22195},  {5, 4, 5668},
	         {1, 5, 25515},  {0, 5, 27863},  {6, 5, 836},    {6, 0, 28699},
	         {2, 6, 23499},  {6, 3, 13871},  {1, 6, 26351},  {5, 7, 6229},
	         {2, 7, 28892},  {1, 7, 31744},  {3, 7, 19264},  {6, 7, 5393},
	         {2, 8, 31998},  {8, 7, 3106},   {3, 8, 22370},  {8, 4, 15003},
	         {8, 6, 8499},   {8, 5, 9335},   {8, 9, 5258},   {9, 2, 37256},
	         {3, 9, 27628},  {7, 9, 8364},   {1, 9, 40108},  {9, 5, 14593},
	         {2, 10, 45922}, {5, 10, 23259}, {9, 10, 8666},  {10, 0, 51122},
	         {10, 3, 36294}, {10, 4, 28927}, {11, 4, 25190}, {11, 9, 4929},
	         {11, 8, 10187}, {11, 6, 18686}, {2, 11, 42185}, {11, 3, 32557},
	         {1, 11, 45037}}
	    ) == 166
	);
	return 0;
}
