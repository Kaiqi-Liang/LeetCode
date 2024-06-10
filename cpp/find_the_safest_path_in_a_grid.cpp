/**
 * @file find_the_safest_path_in_a_grid.cpp
 * @brief https://leetcode.com/problems/find-the-safest-path-in-a-grid/
 * @date 2024-03-17
 */
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <queue>
#include <unordered_set>
#include <vector>

#include "grid.hpp"
using namespace std;

bool is_safe(vector<vector<int>> const &dist, int safe_dist, int n) {
	if (dist[0][0] < safe_dist) return false;
	queue<Point> q{{{0, 0}}};
	unordered_set<Point> visited{{0, 0}};
	while (not q.empty()) {
		auto [curr_row, curr_col] = q.front();
		q.pop();
		if (curr_row == n - 1 and curr_col == n - 1) {
			return true;
		}
		for (auto &&[delta_row, delta_col] : NEIGHBOURS) {
			int new_row = curr_row + delta_row;
			int new_col = curr_col + delta_col;
			if (new_row >= 0 and new_row < n and new_col >= 0 and
			    new_col < n and not visited.contains({new_row, new_col}) and
			    dist[new_row][new_col] >= safe_dist)
			{
				q.emplace(new_row, new_col);
				visited.emplace(new_row, new_col);
			}
		}
	}
	return false;
}

int maximumSafenessFactor(vector<vector<int>> const &grid) {
	int const n = grid.size();
	queue<Point> q;
	unordered_set<Point> visited;
	for (int i = 0; i < n; ++i) {
		for (int j = 0; j < n; ++j) {
			if (grid[i][j]) {
				q.emplace(i, j);
				visited.emplace(i, j);
			}
		}
	}
	vector<vector<int>> dist(n, vector<int>(n, 0));
	while (not q.empty()) {
		auto [curr_row, curr_col] = q.front();
		q.pop();
		for (auto &&[i, j] : NEIGHBOURS) {
			int new_row = curr_row + i;
			int new_col = curr_col + j;
			if (new_row >= 0 and new_row < n and new_col >= 0 and
			    new_col < n and not visited.contains({new_row, new_col}))
			{
				q.emplace(new_row, new_col);
				visited.emplace(new_row, new_col);
				dist[new_row][new_col] = dist[curr_row][curr_col] + 1;
			}
		}
	}
	int l = 2 * n;
	int h = 0;
	for (int i = 0; i < n; ++i) {
		for (int j = 0; j < n; ++j) {
			l = min(l, dist[i][j]);
			h = max(h, dist[i][j]);
		}
	}
	int ans = l;
	while (l <= h) {
		int m = (l + h) / 2;
		if (is_safe(dist, m, n)) {
			ans = m;
			l = m + 1;
		} else {
			h = m - 1;
		}
	}
	return ans;
}

int main() {
	assert(maximumSafenessFactor({{1, 0, 0}, {0, 0, 0}, {0, 0, 1}}) == 0);
	assert(maximumSafenessFactor({{0, 0, 1}, {0, 0, 0}, {0, 0, 0}}) == 2);
	assert(
	    maximumSafenessFactor(
	        {{0, 0, 0, 1}, {0, 0, 0, 0}, {0, 0, 0, 0}, {1, 0, 0, 0}}
	    ) == 2
	);
	return 0;
}
