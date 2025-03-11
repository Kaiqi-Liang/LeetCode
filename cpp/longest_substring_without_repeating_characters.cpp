/**
 * @file longest_substring_without_repeating_characters.cpp
 * @brief https://leetcode.com/problems/longest-substring-without-repeating-characters/
 * @date 2025-03-11
 */
#include <cassert>
#include <string>
#include <unordered_map>
using namespace std;

int lengthOfLongestSubstring(string s) {
	size_t max_count = 0;
	std::unordered_map<char, size_t> char_to_index;
	for (size_t i = 0; i < s.size(); ++i) {
		if (char_to_index.contains(s[i])) {
			i = char_to_index[s[i]];
			char_to_index.clear();
		} else {
            char_to_index[s[i]] = i;
            max_count = std::max(char_to_index.size(), max_count);
        }
	}
	return static_cast<int>(max_count);
}

int main() {
	assert(lengthOfLongestSubstring("tmmzuxt") == 5);
	assert(lengthOfLongestSubstring("abcabcbb") == 3);
	return 0;
}
