/**
 * @file palindrome_partitioning.cpp
 * @brief https://leetcode.com/problems/palindrome-partitioning/
 * @date 2024-03-17
 */
#include <cassert>
#include <string>
#include <vector>

using Partitions = std::vector<std::vector<std::string>>;

auto is_palindrome(std::string const &s, int start, int end) -> bool {
	while (start <= end) {
		if (s[start] != s[end]) return false;
		++start;
		--end;
	}
	return true;
}

auto dfs(
    Partitions &result,
    std::vector<std::string> &partition,
    std::string const &s,
    int start
) -> void {
	if (start == s.size()) result.push_back(partition);
	for (auto i = start; i < s.size(); ++i) {
		if (is_palindrome(s, start, i)) {
			partition.push_back(
			    std::string(s.cbegin() + start, s.cbegin() + i + 1)
			);
			dfs(result, partition, s, i + 1);
			partition.pop_back();
		}
	}
}

auto partition(std::string s) -> Partitions {
	auto result = Partitions{};
	auto partition = std::vector<std::string>{};
	dfs(result, partition, s, 0);
	return result;
}

auto main() -> int {
	assert(partition("aab") == (Partitions{{{"a", "a", "b"}, {"aa", "b"}}}));
	assert(partition("cdd") == (Partitions{{{"c", "d", "d"}, {"c", "dd"}}}));
	return 0;
}
