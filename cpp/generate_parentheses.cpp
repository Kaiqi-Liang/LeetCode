/**
 * @file generate_parentheses.cpp
 * @brief https://leetcode.com/problems/generate-parentheses/
 * @date 2024-04-07
 */
#include <cassert>
#include <string>
#include <unordered_set>
using namespace std;

void recurse(
    string &curr,
    unordered_set<string> &res,
    size_t open,
    size_t close,
    size_t n
) {
	if (open == n and close == n) {
		res.insert(curr);
		return;
	}
	if (open < n) {
		curr += '(';
		recurse(curr, res, open + 1, close, n);
		curr.pop_back();
	}
	if (close < open) {
		curr += ')';
		recurse(curr, res, open, close + 1, n);
		curr.pop_back();
	}
}

unordered_set<string> generateParenthesis(size_t n) {
	unordered_set<string> res;
	string curr;
	recurse(curr, res, 0, 0, n);
	return res;
}

int main() {
	assert(generateParenthesis(0) == unordered_set<string>{""});
	assert(generateParenthesis(1) == unordered_set<string>{"()"});
	assert(generateParenthesis(2) == unordered_set<string>({"()()", "(())"}));
	assert(
	    generateParenthesis(3) ==
	    unordered_set<string>({"((()))", "(()())", "(())()", "()(())", "()()()"}
	    )
	);
	return 0;
}
