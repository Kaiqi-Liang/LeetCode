/**
 * @file word_ladder.cpp
 * @brief https://leetcode.com/problems/word-ladder/
 * @date 2024-02-17
 */
#include <algorithm>
#include <cassert>
#include <queue>
#include <string>
#include <unordered_set>
using namespace std;

unordered_set<string> get_neighbours(string const &word) {
	auto neighbours = unordered_set<string>{};
	for (size_t i = 0; i < word.size(); ++i) {
		for (char j = 'a'; j <= 'z'; ++j) {
			if (word[i] != j) {
				auto copy = word;
				copy[i] = j;
				neighbours.insert(copy);
			}
		}
	}
	return neighbours;
}

int ladderLength(string beginWord, string endWord, vector<string> wordList) {
	auto wordSet = unordered_set<string>{wordList.cbegin(), wordList.cend()};
	auto visited = unordered_set<string>{beginWord};
	auto q = queue<string>{{beginWord, string{}}};
	auto num_words = 1;
	while (not q.empty()) {
		auto const curr = q.front();
		q.pop();
		if (curr.empty()) {
			if (q.empty()) {
				break;
			}
			++num_words;
			q.push(string{});
			continue;
		}
		if (curr == endWord) {
			return num_words;
		}
		for (auto &&neighbour : get_neighbours(curr)) {
			if (not visited.contains(neighbour) && wordSet.contains(neighbour))
			{
				visited.insert(neighbour);
				q.push(neighbour);
			}
		}
	}
	return 0;
}

int main() {
	assert(
	    ladderLength(
	        "ymain",
	        "oecij",
	        {
	            "ymann",
	            "yycrj",
	            "oecij",
	            "ymcnj",
	            "yzcrj",
	            "yycij",
	            "xecij",
	            "yecij",
	            "ymanj",
	            "yzcnj",
	            "ymain",
	        }
	    ) == 10
	);
	return 0;
}
