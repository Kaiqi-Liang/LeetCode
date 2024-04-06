/**
 * @file minimum_remove_to_make_valid_parentheses.cpp
 * @brief https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
 * @date 2024-04-06
 */
#include <cassert>
#include <unordered_set>
#include <string>
#include <stack>
using namespace std;

string minRemoveToMakeValid(string s) {
	stack<int> matching_parenthese;
	unordered_set<int> extra_closing_parenthese;
	int nth_opening_parenthese = 0;
	int nth_closing_parenthese = 0;
	for (char ch : s) {
		if (ch == '(') {
			matching_parenthese.push(nth_opening_parenthese++);
		} else if (ch == ')') {
			if (not matching_parenthese.empty()) matching_parenthese.pop();
			else extra_closing_parenthese.insert(nth_closing_parenthese++);
		}
	}

	unordered_set<int> extra_opening_parenthese;
	while (not matching_parenthese.empty()) {
		extra_opening_parenthese.insert(matching_parenthese.top());
		matching_parenthese.pop();
	}
	nth_opening_parenthese = 0;
	nth_closing_parenthese = 0;
	string res;
	for (char ch : s) {
		if (ch == '(') {
			if (extra_opening_parenthese.contains(nth_opening_parenthese++)) {
				continue;
			}
		} else if (ch == ')') {
			if (extra_closing_parenthese.contains(nth_closing_parenthese++)) {
				continue;
			}
		}
		res.push_back(ch);
	}
	return res;
}

int main() {
	assert(minRemoveToMakeValid("lee(t(c)o)de)") == "lee(t(co)de)");
	assert(minRemoveToMakeValid("))((") == "");
	assert(minRemoveToMakeValid("))(())") == "(())");
	assert(minRemoveToMakeValid("))))") == "");
	assert(minRemoveToMakeValid("((((i") == "i");
	assert(minRemoveToMakeValid(")hi(there))a(bc)d") == "hi(there)a(bc)d");
	assert(minRemoveToMakeValid("())()(((") == "()()");
}
