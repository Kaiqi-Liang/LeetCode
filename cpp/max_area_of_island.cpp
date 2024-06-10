/**
 * @file max_area_of_island.cpp
 * @brief https://leetcode.com/problems/max-area-of-island/
 * @date 2024-04-05
 */
#include <cassert>
#include <unordered_set>
#include <vector>

#include "grid.hpp"
using namespace std;

int dfs(
    vector<vector<int>> const &grid,
    unordered_set<Point> &visited,
    int curr_x,
    int curr_y,
    int area
) {
	visited.emplace(curr_x, curr_y);
	for (auto &&[delta_x, delta_y] : NEIGHBOURS) {
		int new_x = curr_x + delta_x;
		int new_y = curr_y + delta_y;
		if (new_x >= 0 and new_x < grid.size() and new_y >= 0 and
		    new_y < grid.front().size() and grid[new_x][new_y] and
		    not visited.contains({new_x, new_y}))
		{
			area = dfs(grid, visited, new_x, new_y, area + 1);
		}
	}
	return area;
}

int maxAreaOfIsland(vector<vector<int>> const &grid) {
	unordered_set<Point> visited;
	int max_area = 0;
	for (int i = 0; i < grid.size(); i++) {
		for (int j = 0; j < grid.front().size(); j++) {
			if (grid[i][j] and not visited.contains({i, j})) {
				max_area = max(max_area, dfs(grid, visited, i, j, 1));
			}
		}
	}
	return max_area;
}

int main() {
	assert(
	    maxAreaOfIsland(
	        {{0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0},
	         {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
	         {0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0},
	         {0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0},
	         {0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0},
	         {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0},
	         {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
	         {0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0}}
	    ) == 6
	);
	assert(
	    maxAreaOfIsland(
	        {{
	             1,
	             1,
	             1,
	         },
	         {
	             1,
	             1,
	             1,
	         },
	         {
	             1,
	             1,
	             1,
	         },
	         {
	             1,
	             1,
	             1,
	         }}
	    ) == 12
	);
	return 0;
}
