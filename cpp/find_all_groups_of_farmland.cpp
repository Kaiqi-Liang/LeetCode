/**
 * @file find_all_groups_of_farmland.cpp
 * @brief https://leetcode.com/problems/find-all-groups-of-farmland/
 * @date 2024-04-21
 */
#include <cassert>
#include <queue>
#include <unordered_set>
#include <vector>

#include "grid.hpp"
using namespace std;

vector<vector<int>> findFarmland(vector<vector<int>> const &land) {
	unordered_set<Point> visited;
	vector<vector<int>> farmlands;
	for (int row = 0; row < land.size(); ++row) {
		for (int col = 0; col < land.front().size(); ++col) {
			Point bottom_right_corner = {row, col};
			if (not visited.contains({row, col}) and land[row][col]) {
				visited.emplace(row, col);
				queue<Point> q{{{row, col}}};
				while (not q.empty()) {
					auto [curr_row, curr_col] = q.front();
					q.pop();
					bottom_right_corner = {curr_row, curr_col};
					for (auto &&[delta_row, delta_col] : NEIGHBOURS) {
						int new_row = curr_row + delta_row;
						int new_col = curr_col + delta_col;
						if (new_row >= 0 and new_row < land.size() and
						    new_col >= 0 and new_col < land.front().size() and
						    land[new_row][new_col] and
						    not visited.contains({new_row, new_col}))
						{
							q.emplace(new_row, new_col);
							visited.emplace(new_row, new_col);
						}
					}
				}
				farmlands.push_back(
				    {row,
				     col,
				     bottom_right_corner.first,
				     bottom_right_corner.second}
				);
			}
		}
	}
	return farmlands;
}

int main() {
	assert(
	    findFarmland({{1, 0, 0}, {0, 1, 1}, {0, 1, 1}}) ==
	    vector<vector<int>>({{0, 0, 0, 0}, {1, 1, 2, 2}})
	);
	assert(
	    findFarmland({{1, 1}, {1, 1}}) == vector<vector<int>>({{0, 0, 1, 1}})
	);
	assert(findFarmland({{0}}) == vector<vector<int>>{});
	assert(findFarmland({{0}, {0}}) == vector<vector<int>>{});
	assert(
	    findFarmland({{0, 0}, {1, 0}, {1, 0}}) ==
	    vector<vector<int>>({{1, 0, 2, 0}})
	);
	return 0;
}
