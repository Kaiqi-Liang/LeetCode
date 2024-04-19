/**
 * @file island_perimeter.cpp
 * @brief https://leetcode.com/problems/island-perimeter/
 * @date 2024-04-20
 */
#include <cassert>
#include <vector>
using namespace std;

int islandPerimeter(vector<vector<int>> const &grid) {
	int perimeter = 0;
	for (size_t i = 0; i < grid.size(); ++i) {
		for (size_t j = 0; j < grid.front().size(); ++j) {
			if (not grid[i][j]) continue;

			if (i == 0 or (i > 0 and not grid[i - 1][j])) ++perimeter;

			if (i == grid.size() - 1 or
			    (i < grid.size() and not grid[i + 1][j]))
			{
				++perimeter;
			}
			if (j == 0 or (j > 0 and not grid[i][j - 1])) ++perimeter;

			if (j == grid.front().size() - 1 or
			    (j < grid.front().size() and not grid[i][j + 1]))
			{
				++perimeter;
			}
		}
	}
	return perimeter;
}

int main() {
	assert(
	    islandPerimeter({{0, 1, 0, 0}, {1, 1, 1, 0}, {0, 1, 0, 0}, {1, 1, 0, 0}}
	    ) == 16
	);
	assert(islandPerimeter({{1}}) == 4);
	assert(islandPerimeter({{1, 0}}) == 4);
}
