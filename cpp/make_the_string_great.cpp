/**
 * @file make_the_string_great.cpp
 * @brief https://leetcode.com/problems/make-the-string-great/
 * @date 2024-04-06
 */
#include <cassert>
#include <stack>
#include <string>

namespace loop {
	std::string makeGood(std::string s) {
		std::string res;
		for (int i = 0; i < s.size();) {
			res.push_back(s[i]);
			int j = i + 1;
			for (; res.size() > 0 and j < s.size() and
			       tolower(res.back()) == tolower(s[j]) and
			       ((islower(res.back()) and isupper(s[j])) or
			        (isupper(res.back()) and islower(s[j])));
			     ++j)
			{
				--i;
				res.pop_back();
			}
			i = j;
		}
		return res;
	}
} // namespace loop

namespace stack {
	std::string makeGood(std::string s) {
		std::stack<char> string_stack;
		for (char ch : s) {
			if (not string_stack.empty() and abs(string_stack.top() - ch) == 32)
			{
				string_stack.pop();
			} else {
				string_stack.push(ch);
			}
		}
		std::string res;
		while (not string_stack.empty()) {
			res.push_back(string_stack.top());
			string_stack.pop();
		}
		return {res.crbegin(), res.crend()};
	}
} // namespace stack

int main() {
	assert(loop::makeGood("aA") == "");
	assert(loop::makeGood("Pp") == "");
	assert(loop::makeGood("aAB") == "B");
	assert(loop::makeGood("a") == "a");
	assert(loop::makeGood("abBAcC") == "");
	assert(loop::makeGood("abcCBA") == "");
	assert(loop::makeGood("leEeetcode") == "leetcode");
	assert(loop::makeGood("kaAKEPpezKZ") == "zKZ");
	assert(stack::makeGood("aA") == "");
	assert(stack::makeGood("Pp") == "");
	assert(stack::makeGood("aAB") == "B");
	assert(stack::makeGood("a") == "a");
	assert(stack::makeGood("abBAcC") == "");
	assert(stack::makeGood("abcCBA") == "");
	assert(stack::makeGood("leEeetcode") == "leetcode");
	assert(stack::makeGood("kaAKEPpezKZ") == "zKZ");
}
