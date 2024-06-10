/**
 * @file find_if_path_exists_in_graph.cpp
 * @brief https://leetcode.com/problems/find-if-path-exists-in-graph/
 * @date 2024-04-21
 */
#include <cassert>
#include <queue>
#include <stack>
#include <vector>
using namespace std;

bool validPath(
    int n,
    vector<vector<int>> const &edges,
    int source,
    int destination
) {
	vector<vector<int>> graph(n, vector<int>{});
	for (auto &&edge : edges) {
		graph[edge.front()].push_back(edge.back());
		graph[edge.back()].push_back(edge.front());
	}
	vector<bool> visited(n, false);
	stack<int> s{{source}};
	while (not s.empty()) {
		int curr = s.top();
		s.pop();
		if (visited[curr]) continue;
		visited[curr] = true;
		if (curr == destination) return true;
		for (auto &&neighbour : graph[curr]) {
			if (not visited[neighbour]) s.push(neighbour);
		}
	}
	return false;
}

int main() {
	assert(validPath(3, {{0, 1}, {1, 2}, {2, 0}}, 0, 2));
	assert(not validPath(6, {{0, 1}, {0, 2}, {3, 5}, {5, 4}, {4, 3}}, 0, 3));
}
