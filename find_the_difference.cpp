/**
 * @file find_the_difference.cpp
 * @brief https://leetcode.com/problems/find-the-difference/
 * @date 2021-11-18
 */
#include <array>
#include <iostream>
#include <map>
#include <string>

namespace map {
	char findTheDifference(std::string s, std::string t) {
		auto s_occurrence = std::map<char, std::size_t>{};
		auto t_occurrence = std::map<char, std::size_t>{};
		for (auto &&c : s) {
			s_occurrence[c]++;
		}
		for (auto &&c : t) {
			t_occurrence[c]++;
			if (t_occurrence[c] > s_occurrence[c]) return c;
		}
		return 0;
	}
} // namespace map

namespace array {
	char findTheDifference(std::string s, std::string t) {
		auto occurrences = std::array<int, 26>{};
		for (auto &&c : s) {
			occurrences[c - 'a']++;
		}
		for (auto &&c : t) {
			occurrences[c - 'a']--;
			if (occurrences[c - 'a'] == -1) return c;
		}
		return 0;
	}
} // namespace array

auto main() -> int {
	assert(array::findTheDifference("abcd", "abcde") == map::findTheDifference("abcd", "abcde"));
	assert(array::findTheDifference("", "y") == map::findTheDifference("", "y"));
	assert(array::findTheDifference("a", "aa") == map::findTheDifference("a", "aa"));
	assert(array::findTheDifference("ae", "aea") == map::findTheDifference("ae", "aea"));
	assert(array::findTheDifference("abc", "bacc") == 'c');
	assert(map::findTheDifference("aaaabc", "abaacc") == 'c');
	return 0;
}
