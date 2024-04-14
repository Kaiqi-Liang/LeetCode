/**
 * @file isomorphic_strings.cpp
 * @brief https://leetcode.com/problems/isomorphic-strings/
 * @date 2024-04-14
 */
#include <cassert>
#include <string>
#include <unordered_map>
#include <unordered_set>
using namespace std;

bool isIsomorphic(string s, string t) {
	unordered_map<char, char> mappings;
	unordered_set<char> mapped_characters;
	for (size_t i = 0; i < s.size(); ++i) {
		if ((mappings.contains(s[i]) and mappings[s[i]] != t[i]) or
		    (not mappings.contains(s[i]) and mapped_characters.contains(t[i])))
			return false;
		mapped_characters.insert(mappings[s[i]] = t[i]);
	}
	return true;
}

int main() {
	assert(isIsomorphic("egg", "add"));
	assert(not isIsomorphic("foo", "bar"));
	assert(isIsomorphic("paper", "title"));
	assert(not isIsomorphic("badc", "baba"));
}
