/**
 * @file longest_common_subsequence.cpp
 * @brief https://leetcode.com/problems/longest-common-subsequence/
 * @date 2021-10-23
 */
#include <iostream>
#include <string>
#include <vector>

auto lengthOfLCS(std::string text1, std::string text2) -> std::size_t {
	auto opt = std::vector<std::vector<std::size_t>>(text1.size() + 1, std::vector<std::size_t>(text2.size() + 1));
	for (auto i = std::size_t{0}; i < text1.size(); ++i) {
		for (auto j = std::size_t{0}; j < text2.size(); ++j) {
			opt[i + 1][j + 1] = text1[i] == text2[j] ? opt[i][j] + 1 : std::max(opt[i][j + 1], opt[i + 1][j]);
		}
	}
#ifdef DEBUG
	for (auto i = std::size_t{0}; i <= text1.size(); ++i) {
		for (auto j = std::size_t{0}; j <= text2.size(); ++j) {
			std::cout << opt[i][j] << ' ';
		}
		std::cout << '\n';
	}
#endif
	return opt[text1.size()][text2.size()];
}

auto main() -> int {
	assert(lengthOfLCS("abcbdab", "bdcaba") == 4);
	assert(lengthOfLCS("abcde", "ace") == 3);
	assert(lengthOfLCS("abc", "abc") == 3);
	assert(lengthOfLCS("abc", "def") == 0);
	return 0;
}
