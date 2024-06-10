/**
 * @file encrypt_and_decrypt_strings.cpp
 * @brief https://leetcode.com/problems/encrypt-and-decrypt-strings/
 * @date 2024-03-31
 */
#include <cassert>
#include <string>
#include <vector>
using namespace std;

class Encrypter {
private:
	vector<char> keys_;
	vector<string> values_;
	vector<string> dictionary_;

public:
	Encrypter(
	    vector<char> const &keys,
	    vector<string> const &values,
	    vector<string> const &dictionary
	)
	: keys_{keys}
	, values_{values}
	, dictionary_{dictionary} {}

	string encrypt(string word1) {
		string encrypted_word;
		for (char ch : word1) {
			auto iter = find(keys_.cbegin(), keys_.cend(), ch);
			if (iter == keys_.cend()) return "";
			encrypted_word += values_[iter - keys_.cbegin()];
		}
		return encrypted_word;
	}

	int decrypt(string word2) {
		int count = 0;
		for (auto &&word : dictionary_) {
			if (encrypt(word) == word2) ++count;
		}
		return count;
	}
};

int main() {
	{
		Encrypter *obj =
		    new Encrypter({'b'}, {"ca"}, {"aaa", "cacbc", "bbaba", "bb"});
		assert(obj->encrypt("bbb") == "cacaca");
		assert(obj->decrypt("cacaca") == 0);
		delete obj;
	}
	{
		Encrypter *obj = new Encrypter(
		    {'a', 'b', 'c', 'd'},
		    {"ei", "zf", "ei", "am"},
		    {"abcd", "acbd", "adbc", "badc", "dacb", "cadb", "cbda", "abad"}
		);
		assert(obj->encrypt("abcd") == "eizfeiam");
		assert(obj->decrypt("eizfeiam") == 2);
		delete obj;
	}
	return 0;
}
