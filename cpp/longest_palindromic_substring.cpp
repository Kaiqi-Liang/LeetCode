/**
 * @file longest_palindromic_substring.cpp
 * @brief https://leetcode.com/problems/longest-palindromic-substring/
 * @date 2022-02-19
 */
#include <string>
#include <vector>

auto longestPalindrome(std::string const& s) -> std::string {
	auto max = 1;
	auto start = 0;
	for (auto i = 0; i < s.size(); ++i) {
		for (auto j = 0; j < 2; ++j) {
			for (auto l = i - j, r = i + 1; l >= 0 and r < s.size(); --l, ++r) {
				if (s[l] == s[r]) {
					auto const length = r - l + 1;
					if (length > max) {
						max = length;
						start = l;
					}
				} else {
					break;
				}
			}
		}
	}
	return s.substr(start, max);
}

auto main() -> int {
	assert(longestPalindrome("babad") == "bab");
	assert(longestPalindrome("cbbd") == "bb");
	return 0;
}
