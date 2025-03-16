/**
 * @file split_a_string_into_the_max_number_of_unique_substrings.cpp
 * @brief https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/
 * @date 2025-03-17
 */
#include <cassert>
#include <stack>
#include <string>
#include <unordered_set>
using namespace std;

class Solution {
public:
	int maxUniqueSplit(string s) {
		backtrack(s, 0);
		return max_unique_split_;
	}

private:
	size_t max_unique_split_ = 0;
	unordered_set<string> split_set_;
	stack<string> split_stack_;
	void backtrack(string const &s, size_t index) {
		if (index >= s.size()) {
			assert(split_set_.size() == split_stack_.size());
			max_unique_split_ = max(max_unique_split_, split_set_.size());
		} else {
			for (size_t i = index + 1; i <= s.size(); ++i) {
				string split = s.substr(index, i - index);
				if (not split_set_.contains(split)) {
					split_set_.insert(split);
					split_stack_.push(split);
					backtrack(s, i);
					split_set_.erase(split_stack_.top());
					split_stack_.pop();
				}
			}
		}
	}
};

int main() {
	assert(Solution().maxUniqueSplit("aa") == 1);
	assert(Solution().maxUniqueSplit("aba") == 2);
	assert(Solution().maxUniqueSplit("ababccc") == 5);
	return 0;
}
