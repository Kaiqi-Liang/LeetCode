/**
 * @file minimum_moves_to_convert_string.cpp
 * @brief https://leetcode.com/problems/minimum-moves-to-convert-string/
 * @date 2024-12-08
 */
#include <string>
#include <cassert>
size_t minimumMoves(std::string s) {
	size_t count = 0;
	for (size_t index = 0; index < s.length(); ++index) {
		if (s[index] == 'X') {
			++count;
			index += 2;
		}
	}
	return count;
}

int main() {
	assert(minimumMoves("XXX") == 1);
	assert(minimumMoves("XXOX") == 2);
	assert(minimumMoves("OOOO") == 0);
}
