/**
 * @file minimum_height_trees.cpp
 * @brief https://leetcode.com/problems/minimum-height-trees/
 * @date 2024-04-23
 */
#include <algorithm>
#include <cassert>
#include <queue>
#include <unordered_set>
#include <vector>
using namespace std;

namespace brute_force {

	int find_tree_height(
	    int node,
	    vector<vector<int>> const &graph,
	    vector<bool> &visited
	) {
		int tree_height = 0;
		for (auto &&neighbour : graph[node]) {
			if (not visited[neighbour]) {
				visited[neighbour] = true;
				tree_height =
				    max(tree_height,
				        find_tree_height(neighbour, graph, visited));
			}
		}
		return tree_height + 1;
	}

	vector<int> findMinHeightTrees(int n, vector<vector<int>> const &edges) {
		vector<vector<int>> graph(n);
		for (auto &&edge : edges) {
			graph[edge.front()].push_back(edge.back());
			graph[edge.back()].push_back(edge.front());
		}
		vector<int> tree_heights(n);
		for (int i = 0; i < n; ++i) {
			vector<bool> visited(n, false);
			visited[i] = true;
			tree_heights[i] = find_tree_height(i, graph, visited);
		}
		vector<int> min_height_trees;
		int min_height =
		    *min_element(tree_heights.cbegin(), tree_heights.cend());
		for (int i = 0; i < tree_heights.size(); ++i) {
			if (min_height == tree_heights[i]) {
				min_height_trees.push_back(i);
			}
		}
		return min_height_trees;
	}
} // namespace brute_force

namespace find_middle {
	vector<int> findMinHeightTrees(int n, vector<vector<int>> const &edges) {
		if (n == 1) return {0};
		vector<unordered_set<int>> graph(n);
		for (auto &&edge : edges) {
			graph[edge.front()].insert(edge.back());
			graph[edge.back()].insert(edge.front());
		}
		vector<int> degrees(n);
		queue<int> s{{-1}};
		for (int i = 0; i < graph.size(); ++i) {
			degrees[i] = graph[i].size();
			if (degrees[i] == 1) s.push(i);
		}
		vector<int> res;
		while (true) {
			int node = s.front();
			s.pop();
			if (node == -1) {
				if (n <= 2) {
					break;
				}
				s.push(-1);
				continue;
			}
			int neighbour = *graph[node].begin();
			graph[node].erase(neighbour);
			graph[neighbour].erase(node);
			--n;
			if (graph[neighbour].size() == 1) {
				s.push(neighbour);
			}
		}
		while (not s.empty()) {
			res.push_back(s.front());
			s.pop();
		}
		return res;
	}
} // namespace find_middle

int main() {
	assert(find_middle::findMinHeightTrees(1, {}) == vector<int>{0});
	assert(find_middle::findMinHeightTrees(2, {{0, 1}}) == vector<int>({0, 1}));
	assert(
	    find_middle::findMinHeightTrees(
	        6,
	        {{0, 1}, {0, 2}, {0, 3}, {3, 4}, {4, 5}}
	    ) == vector<int>{3}
	);
	assert(
	    find_middle::findMinHeightTrees(4, {{1, 0}, {1, 2}, {1, 3}}) ==
	    vector<int>{1}
	);
	assert(
	    find_middle::findMinHeightTrees(
	        6,
	        {{3, 0}, {3, 1}, {3, 2}, {3, 4}, {5, 4}}
	    ) == vector<int>({3, 4})
	);
	assert(brute_force::findMinHeightTrees(1, {}) == vector<int>{0});
	assert(brute_force::findMinHeightTrees(2, {{0, 1}}) == vector<int>({0, 1}));
	assert(
	    brute_force::findMinHeightTrees(
	        6,
	        {{0, 1}, {0, 2}, {0, 3}, {3, 4}, {4, 5}}
	    ) == vector<int>{3}
	);
	assert(
	    brute_force::findMinHeightTrees(4, {{1, 0}, {1, 2}, {1, 3}}) ==
	    vector<int>{1}
	);
	assert(
	    brute_force::findMinHeightTrees(
	        6,
	        {{3, 0}, {3, 1}, {3, 2}, {3, 4}, {5, 4}}
	    ) == vector<int>({3, 4})
	);
	return 0;
}
