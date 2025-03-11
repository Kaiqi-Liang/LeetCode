/**
 * @file bus_routes.cpp
 * @brief https://leetcode.com/problems/bus-routes/
 * @date 2025-03-11
 */
#include <cassert>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;

namespace {
	bool is_connected(
	    size_t i,
	    size_t j,
	    vector<unordered_set<size_t>> const &route_set
	) {
		for (size_t bus_stop : route_set[i]) {
			if (route_set[j].contains(bus_stop)) {
				return true;
			}
		}
		return false;
	}
} // namespace

int numBusesToDestination(
    vector<vector<size_t>> const &routes,
    size_t source,
    size_t target
) {
	if (source == target) return 0;
	vector<unordered_set<size_t>> route_set;
	transform(
	    routes.cbegin(),
	    routes.cend(),
	    back_inserter(route_set),
	    [](vector<size_t> const &route) {
		    return unordered_set<size_t>(route.cbegin(), route.cend());
	    }
	);
	vector<unordered_set<size_t>> graph(routes.size());
	queue<size_t> q;
	unordered_map<size_t, size_t> visited;
	for (size_t i = 0; i < routes.size(); ++i) {
		if (route_set[i].contains(source)) {
			q.push(i);
			visited[i] = 1;
		}
		for (size_t j = 0; j < routes.size(); ++j) {
			if (i != j and is_connected(i, j, route_set)) {
				graph[i].insert(j);
				graph[j].insert(i);
			}
		}
	}
	while (not q.empty()) {
		size_t curr = q.front();
		q.pop();
		if (route_set[curr].contains(target))
			return static_cast<int>(visited[curr]);
		for (size_t neighbour : graph[curr]) {
			if (not visited.contains(neighbour)) {
				visited[neighbour] = visited[curr] + 1;
				q.push(neighbour);
			}
		}
	}
	return -1;
}

int main() {
	assert(numBusesToDestination({{1, 2, 7}, {3, 6, 7}}, 1, 6) == 2);
	assert(
	    numBusesToDestination(
	        {{7, 12}, {4, 5, 15}, {6}, {15, 19}, {9, 12, 13}},
	        15,
	        12
	    ) == -1
	);
	assert(numBusesToDestination({{1, 7}, {3, 5}}, 5, 5) == 0);
	return 0;
}
