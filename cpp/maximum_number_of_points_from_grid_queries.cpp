/**
 * @file maximum_number_of_points_from_grid_queries.cpp
 * @brief
 * https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/
 * @date 2025-03-29
 */
#include <algorithm>
#include <cassert>
#include <queue>
#include <unordered_set>
#include <vector>

#include "grid.hpp"
using namespace std;

using IndexQueryPair = pair<size_t, int>;
using ValuePointPair = pair<int, Point>;

vector<int>
maxPoints(vector<vector<int>> const &grid, vector<int> const &queries) {
	int const n = grid.size();
	int const m = grid.front().size();
	vector<IndexQueryPair> sorted_queries_with_indices;
	size_t i = 0;
	ranges::transform(
	    queries,
	    back_inserter(sorted_queries_with_indices),
	    [&i](int query) { return make_pair(query, i++); }
	);
	ranges::sort(sorted_queries_with_indices);
	vector<int> answer(sorted_queries_with_indices.size());
	unordered_set<Point> visited;
	priority_queue<
	    ValuePointPair,
	    vector<ValuePointPair>,
	    greater<ValuePointPair>>
	    min_heap;
	min_heap.emplace(grid[0][0], make_pair(0, 0));
	for (auto &&[query, index] : sorted_queries_with_indices) {
		while (not min_heap.empty() and min_heap.top().first < query) {
			auto [curr_val, curr_point] = min_heap.top();
			auto [curr_row, curr_col] = curr_point;
			min_heap.pop();
			visited.emplace(curr_row, curr_col);
			for (auto &&[delta_row, delta_col] : NEIGHBOURS) {
				int new_row = curr_row + delta_row;
				int new_col = curr_col + delta_col;
				if (new_row >= 0 and new_row < n and new_col >= 0 and
				    new_col < m and not visited.contains({new_row, new_col}))
				{
					min_heap.emplace(
					    grid[new_row][new_col],
					    make_pair(new_row, new_col)
					);
				}
			}
		}
		answer[index] = visited.size();
	}
	return answer;
}

int main() {
	assert(
	    maxPoints({{1, 2, 3}, {2, 5, 7}, {3, 5, 1}}, {5, 6, 2}) ==
	    vector({5, 8, 1})
	);
	assert(maxPoints({{5, 2, 1}, {1, 1, 2}}, {3}) == vector({0}));
	assert(maxPoints({{5, 2, 1}, {1, 1, 2}}, {3, 5}) == vector({0, 0}));
	assert(maxPoints({{5, 2, 1}, {1, 1, 2}}, {3, 6}) == vector({0, 6}));
	return 0;
}
