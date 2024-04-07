/**
 * @file valid_parenthesis_string.cpp
 * @brief https://leetcode.com/problems/valid-parenthesis-string/
 * @date 2024-04-07
 */
#include <cassert>
#include <string>
using namespace std;

bool checkValidString(string s) {
	int left_min = 0, left_max = 0;
	for (char ch : s) {
		switch (ch) {
			case '(':
                ++left_min;
                ++left_max;
				break;
			case ')':
                left_min = max(left_min - 1, 0);
                if (--left_max < 0) return false;
				break;
			case '*':
                left_min = max(left_min - 1, 0);
                ++left_max;
				break;
		}
	}
    return left_min == 0;
}

int main() {
	assert(checkValidString("()"));
	assert(checkValidString("(*)"));
	assert(checkValidString("((*)"));
	assert(checkValidString("(*))"));
	assert(checkValidString("(*()"));
	assert(not checkValidString("(*(()"));
	assert(checkValidString("(*(()*"));
	assert(checkValidString("((*))"));
	return 0;
}
