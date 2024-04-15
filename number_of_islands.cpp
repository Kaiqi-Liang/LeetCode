/**
 * @file number_of_islands.cpp
 * @brief https://leetcode.com/problems/number-of-islands/
 * @date 2024-04-15
 */
#include <cassert>
#include <unordered_set>
#include <vector>

#include "grid.hpp"
using namespace std;

void dfs(
    vector<vector<char>> const &grid,
    unordered_set<Point> &visited,
    int curr_x,
    int curr_y
) {
	visited.emplace(curr_x, curr_y);
	for (auto &&[delta_x, delta_y] : NEIGHBOURS) {
		int new_x = curr_x + delta_x;
		int new_y = curr_y + delta_y;
		if (new_x >= 0 and new_x < grid.size() and new_y >= 0 and
		    new_y < grid.front().size() and grid[new_x][new_y] == '1' and
		    not visited.contains({new_x, new_y}))
		{
			dfs(grid, visited, new_x, new_y);
		}
	}
}

int numIslands(vector<vector<char>> const &grid) {
	int num = 0;
	unordered_set<Point> visited;
	for (int i = 0; i < grid.size(); ++i) {
		for (int j = 0; j < grid.front().size(); ++j) {
			if (grid[i][j] == '1' and not visited.contains({i, j})) {
				dfs(grid, visited, i, j);
				++num;
			}
		}
	}
	return num;
}

int main() {
	assert(
	    numIslands(
	        {{'1', '1', '1', '1', '0'},
	         {'1', '1', '0', '1', '0'},
	         {'1', '1', '0', '0', '0'},
	         {'0', '0', '0', '0', '0'}}
	    ) == 1
	);
	assert(
	    numIslands(
	        {{'1', '1', '0', '0', '0'},
	         {'1', '1', '0', '0', '0'},
	         {'0', '0', '1', '0', '0'},
	         {'0', '0', '0', '1', '1'}}
	    ) == 3
	);
	return 0;
}
