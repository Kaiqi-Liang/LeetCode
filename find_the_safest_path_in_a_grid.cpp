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
using namespace std;

constexpr array<pair<size_t, size_t>, 4> neighbours = {{
    {0, 1},
    {0, -1},
    {1, 0},
    {-1, 0},
}};

struct pair_hash {
	inline size_t operator()(const pair<size_t, size_t> &v) const {
		return v.first * 31 + v.second;
	}
};

bool is_safe(vector<vector<size_t>> const &dist, size_t safe_dist, size_t n) {
	if (dist[0][0] < safe_dist) return false;
	queue<pair<size_t, size_t>> q{{{0, 0}}};
	unordered_set<pair<size_t, size_t>, pair_hash> visited{{0, 0}};
	while (not q.empty()) {
		auto [curr_row, curr_col] = q.front();
		q.pop();
		if (curr_row == n - 1 and curr_col == n - 1) {
			return true;
		}
		for (auto &&[i, j] : neighbours) {
			size_t new_row = curr_row + i;
			size_t new_col = curr_col + j;
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

size_t maximumSafenessFactor(vector<vector<size_t>> const &grid) {
	size_t const n = grid.size();
	queue<pair<size_t, size_t>> q;
	unordered_set<pair<size_t, size_t>, pair_hash> visited;
	for (size_t i = 0; i < n; ++i) {
		for (size_t j = 0; j < n; ++j) {
			if (grid[i][j]) {
				q.emplace(i, j);
				visited.emplace(i, j);
			}
		}
	}
	vector<vector<size_t>> dist(n, vector<size_t>(n, 0));
	while (not q.empty()) {
		auto [curr_row, curr_col] = q.front();
		q.pop();
		for (auto &&[i, j] : neighbours) {
			size_t new_row = curr_row + i;
			size_t new_col = curr_col + j;
			if (new_row >= 0 and new_row < n and new_col >= 0 and
			    new_col < n and not visited.contains({new_row, new_col}))
			{
				q.emplace(new_row, new_col);
				visited.emplace(new_row, new_col);
				dist[new_row][new_col] = dist[curr_row][curr_col] + 1;
			}
		}
	}
	size_t l = 2 * n;
	size_t h = 0;
	for (size_t i = 0; i < n; ++i) {
		for (size_t j = 0; j < n; ++j) {
            l = min(l, dist[i][j]);
            h = max(h, dist[i][j]);
		}
	}
	size_t ans = l;
	while (l <= h) {
		size_t m = (l + h) / 2;
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
