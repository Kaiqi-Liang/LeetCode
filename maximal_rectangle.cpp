/**
 * @file maximal_rectangle.cpp
 * @brief https://leetcode.com/problems/maximal-rectangle/
 * @date 2024-04-22
 */
#include <cassert>
#include <vector>
using namespace std;

size_t maximalRectangle(vector<vector<char>> const &matrix) {
	size_t rows = matrix.size();
	size_t cols = matrix.front().size();
	vector<size_t> prev_height(cols, 0), prev_left(cols, 0),
	    prev_right(cols, cols - 1), curr_height(cols, 0), curr_left(cols, 0),
	    curr_right(cols, cols - 1);
	bool alternate = true;
	size_t max_area = 0;
	for (size_t row = 0; row < rows; ++row) {
		size_t left_bound = 0;
		for (size_t col = 0; col < cols; ++col) {
			if (matrix[row][col] == '1') {
				(alternate ? curr_height : prev_height)[col] =
				    (alternate ? prev_height : curr_height)[col] + 1;
				(alternate ? curr_left : prev_left)[col] =
				    max(left_bound, (alternate ? prev_left : curr_left)[col]);
			} else {
				left_bound = col + 1;
				(alternate ? curr_height : prev_height)[col] = 0;
				(alternate ? curr_left : prev_left)[col] = 0;
			}
		}
		size_t right_bound = cols - 1;
		for (size_t col = cols; col > 0; --col) {
			if (matrix[row][col - 1] == '1') {
				(alternate ? curr_right : prev_right)[col - 1] =
				    min(right_bound,
				        (alternate ? prev_right : curr_right)[col - 1]);
			} else {
				right_bound = col - 2;
				(alternate ? curr_right : prev_right)[col - 1] = cols - 1;
			}
		}
		for (size_t col = 0; col < cols; ++col) {
			max_area = max(
			    max_area,
			    alternate
			        ? curr_height[col] * (curr_right[col] - curr_left[col] + 1)
			        : prev_height[col] * (prev_right[col] - prev_left[col] + 1)
			);
		}
		alternate = !alternate;
	}
	return max_area;
}

int main() {
	assert(
	    maximalRectangle(
	        {{'0', '0', '0', '1', '0', '1', '1', '1'},
	         {'0', '1', '1', '0', '0', '1', '0', '1'},
	         {'1', '0', '1', '1', '1', '1', '0', '1'},
	         {'0', '0', '0', '1', '0', '0', '0', '0'},
	         {'0', '0', '1', '0', '0', '0', '1', '0'},
	         {'1', '1', '1', '0', '0', '1', '1', '1'},
	         {'1', '0', '0', '1', '1', '0', '0', '1'},
	         {'0', '1', '0', '0', '1', '1', '0', '0'},
	         {'1', '0', '0', '1', '0', '0', '0', '0'}}
	    ) == 4
	);
	assert(
	    maximalRectangle(
	        {{'1', '0', '1', '0', '0'},
	         {'1', '0', '1', '1', '1'},
	         {'1', '1', '1', '1', '1'},
	         {'1', '0', '0', '1', '0'}}
	    ) == 6
	);
	assert(
	    maximalRectangle(
	        {{'0', '0', '1', '0', '0'},
	         {'0', '1', '1', '1', '0'},
	         {'1', '1', '1', '1', '1'},
	         {'0', '0', '1', '1', '0'}}
	    ) == 6
	);
	assert(maximalRectangle({{'0'}}) == 0);
	assert(maximalRectangle({{'1'}}) == 1);
	return 0;
}
