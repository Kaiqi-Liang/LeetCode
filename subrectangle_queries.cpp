/**
 * @file subrectangle_queries.cpp
 * @brief https://leetcode.com/problems/subrectangle-queries/
 * @date 2024-03-19
 */
#include <cassert>
#include <vector>
using namespace std;

class SubrectangleQueries {
private:
	vector<vector<int>> rectangle_;

public:
	SubrectangleQueries(vector<vector<int>> const &rectangle)
	: rectangle_{rectangle} {}

	void
	updateSubrectangle(int row1, int col1, int row2, int col2, int newValue) {
		for (int i = row1; i <= row2; ++i) {
			for (int j = col1; j <= col2; ++j) {
				rectangle_[i][j] = newValue;
			}
		}
	}

	int getValue(int row, int col) {
		return rectangle_[row][col];
	}
};

int main() {
	SubrectangleQueries *obj =
	    new SubrectangleQueries({{{{1, 2, 1}, {4, 3, 4}, {3, 2, 1}, {1, 1, 1}}}}
	    );
	obj->updateSubrectangle(0, 0, 3, 2, 5);
	assert(obj->getValue(0, 2) == 5);
	delete obj;
	return 0;
}
