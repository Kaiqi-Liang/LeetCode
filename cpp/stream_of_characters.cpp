/**
 * @file stream_of_characters.cpp
 * @brief https://leetcode.com/problems/stream-of-characters/
 * @date 2024-03-30
 */
#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

struct trie {
	bool is_end_word;
	unordered_map<char, trie *> children;
	~trie() {
		for (auto &&child : children) {
			delete child.second;
		}
	}
};

class StreamChecker {
private:
	unordered_map<char, trie *> trie_;
	vector<char> stream_;

public:
	StreamChecker(vector<string> const &words) {
		for (auto &&word : words) {
			unordered_map<char, trie *> *curr = &trie_;
			for (int i = word.size() - 1; i >= 0; --i) {
				if (curr->contains(word[i])) {
					if (i == 0) curr->at(word[i])->is_end_word = true;
				} else {
					curr->emplace(word[i], new trie{i == 0});
				}
				curr = &curr->at(word[i])->children;
			}
		}
	}

	bool query(char letter) {
		stream_.push_back(letter);
		unordered_map<char, trie *> *curr = &trie_;
		for (int i = stream_.size() - 1; i >= 0; --i) {
			if (curr->contains(stream_[i]) and
			    curr->at(stream_[i])->is_end_word)
			{
				return true;
			} else if (not curr->contains(stream_[i])) {
				return false;
			}
			curr = &curr->at(stream_[i])->children;
		}
		return false;
	}
};

int main() {
	{
		StreamChecker *obj = new StreamChecker({"abc", "xyz", "l"});
		assert(not obj->query('a'));
		assert(not obj->query('b'));
		assert(obj->query('c'));
		assert(not obj->query('x'));
		assert(not obj->query('y'));
		assert(not obj->query('x'));
		assert(not obj->query('z'));
		assert(not obj->query('a'));
		assert(not obj->query('b'));
		assert(not obj->query('a'));
		assert(not obj->query('b'));
		assert(obj->query('c'));
		delete obj;
	}
	{
		StreamChecker *obj =
		    new StreamChecker({"ab", "ba", "aaab", "abab", "baa"});
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(obj->query('a'));
		assert(obj->query('b'));
		assert(obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('b'));
		assert(obj->query('a'));
		assert(obj->query('b'));
		assert(obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('b'));
		assert(obj->query('a'));
		assert(obj->query('b'));
		assert(obj->query('a'));
		assert(obj->query('b'));
		assert(obj->query('a'));
		assert(obj->query('a'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(obj->query('a'));
		assert(obj->query('a'));
		assert(not obj->query('a'));
		delete obj;
	}
	{
		StreamChecker *obj =
		    new StreamChecker({"abaa", "abaab", "aabbb", "bab", "ab"});
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('b'));
		assert(obj->query('b'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('b'));
		assert(not obj->query('a'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('b'));
		assert(obj->query('b'));
		assert(not obj->query('a'));
		assert(obj->query('b'));
		assert(not obj->query('a'));
	}
}
