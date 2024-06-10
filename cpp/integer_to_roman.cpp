/**
 * @file integer_to_roman.cpp
 * @brief https://leetcode.com/problems/integer-to-roman/
 * @date 2024-04-01
 */
#include <cassert>
#include <stdexcept>
#include <string>
using namespace std;

string composeRomanLetters(int digit, char first, char second, char third) {
	switch (digit) {
		case 0:
		case 1:
		case 2:
		case 3: {
			string roman;
			for (int i = 0; i < digit; ++i) roman += first;
			return roman;
		}
		case 4:
			return string{first} + string{second};
		case 5:
		case 6:
		case 7:
		case 8: {
			string roman{second};
			for (int i = 0; i < digit - 5; ++i) roman += first;
			return roman;
		}
		case 9:
			return string{first} + string{third};
		default:
			throw runtime_error{"Invalid digit"};
	}
}

string intToRoman(int num) {
	string roman;
	for (int i = 0; i < num / 1000; ++i) roman += 'M';
	return roman + composeRomanLetters(num % 1000 / 100, 'C', 'D', 'M') +
	       composeRomanLetters(num % 100 / 10, 'X', 'L', 'C') +
	       composeRomanLetters(num % 10, 'I', 'V', 'X');
}

int main() {
	assert(intToRoman(3) == "III");
	assert(intToRoman(58) == "LVIII");
	assert(intToRoman(1994) == "MCMXCIV");
	return 0;
}
