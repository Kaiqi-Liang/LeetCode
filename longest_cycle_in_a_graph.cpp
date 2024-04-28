#include <cassert>
#include <vector>
using namespace std;

int dfs(
    vector<int> const &graph,
    int node,
    vector<int> &depths,
    vector<bool> &visited
) {
	visited[node] = true;
	if (graph[node] > -1) {
		int const neighbour = graph[node];
		if (depths[neighbour] > 0) {
			return depths[node] - depths[neighbour] + 1;
		}
		depths[neighbour] = depths[node] + 1;
		int const res =
		    visited[neighbour] ? -1 : dfs(graph, neighbour, depths, visited);
		depths[neighbour] = 0;
		return res;
	}
	depths[node] = 0;
	return -1;
}

int longestCycle(vector<int> const &edges) {
	int n = static_cast<int>(edges.size());
	vector<int> graph(n, -1);
	for (int i = 0; i < n; ++i) graph[i] = edges[i];
	int max_cycle = -1;
	vector<bool> visited(n, false);
	vector<int> depths(n, 0);
	for (int i = 0; i < n; ++i) {
		if (visited[i]) continue;
		++depths[i];
		max_cycle = max(max_cycle, dfs(graph, i, depths, visited));
		depths[i] = 0;
	}
	return max_cycle;
}

int main() {
	assert(longestCycle({3, 4, 0, 2, -1, 2}) == 3);
	assert(longestCycle({3, 3, 4, 2, 3}) == 3);
	assert(longestCycle({2, -1, 3, 1}) == -1);
	return 0;
}
