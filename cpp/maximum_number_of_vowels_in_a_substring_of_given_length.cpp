/**
 * @file maximum_number_of_vowels_in_a_substring_of_given_length.cpp
 * @brief https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/
 * @date 2025-03-11
 */
#include <cassert>
#include <string>
#include <unordered_set>
using namespace std;

int maxVowels(string s, size_t k) {
	unordered_set<char> vowels{'a', 'e', 'i', 'o', 'u'};
	int num_vowels = 0;
	int max_vowels = 0;
	for (size_t i = 0; i < s.size(); ++i) {
		if (vowels.contains(s[i])) ++num_vowels;
		if (i >= k and vowels.contains(s[i - k])) --num_vowels;
		max_vowels = max(max_vowels, num_vowels);
	}
	return max_vowels;
}

int main() {
	assert(maxVowels("abciiidef", 3) == 3);
	assert(maxVowels("aeiou", 2) == 2);
	return 0;
}
