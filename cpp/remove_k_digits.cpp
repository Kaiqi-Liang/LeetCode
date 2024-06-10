/**
 * @file remove_k_digits.cpp
 * @brief https://leetcode.com/problems/remove-k-digits/
 * @date 2024-02-28
 */
#include <cassert>
#include <stack>
#include <string>
using namespace std;

// O(n^2)
namespace brute_force {
	string removeKdigits(string num, size_t k) {
		if (num.size() == k) {
			return "0";
		}
		for (size_t i = 0; i < k; ++i) {
			size_t j = 1;
			bool erased = false;
			for (; j < num.size(); ++j) {
				if (num[j] < num[j - 1]) {
					num.erase(j - 1, 1);
					erased = true;
					break;
				}
			}
			if (not erased) {
				num.erase(num.size() - 1, 1);
			}
		}
		auto first_not_zero = num.find_first_not_of("0");
		if (first_not_zero != string::npos) {
			num.erase(0, first_not_zero);
		} else {
			return "0";
		}
		return num;
	}
} // namespace brute_force

// O(n)
namespace monotonic_stack {
	string removeKdigits(string num, int k) {
		stack<char> s{{num[0]}};
		for (size_t i = 1; i < num.size(); ++i) {
			while (k > 0 and not s.empty() and num[i] < s.top()) {
				s.pop();
				--k;
			}
			s.push(num[i]);
		}
		string res;
		while (k > 0) {
			s.pop();
			--k;
		}
		while (not s.empty()) {
			res += s.top();
			s.pop();
		}
		res = string(
		    find_if(
		        res.crbegin(),
		        res.crend(),
		        [](char ch) { return ch != '0'; }
		    ),
		    res.crend()
		);
		if (res.empty()) {
			return "0";
		}
		return res;
	}
} // namespace monotonic_stack

int main() {
	assert(monotonic_stack::removeKdigits("1432219", 3) == "1219");
	assert(monotonic_stack::removeKdigits("10200", 1) == "200");
	assert(monotonic_stack::removeKdigits("10", 2) == "0");
	assert(monotonic_stack::removeKdigits("10", 1) == "0");
	assert(monotonic_stack::removeKdigits("112", 1) == "11");
	assert(monotonic_stack::removeKdigits("122", 1) == "12");
	assert(monotonic_stack::removeKdigits("122", 2) == "1");
	assert(monotonic_stack::removeKdigits("231", 2) == "1");
	assert(monotonic_stack::removeKdigits("1234567890", 9) == "0");
	assert(brute_force::removeKdigits("1432219", 3) == "1219");
	assert(brute_force::removeKdigits("10200", 1) == "200");
	assert(brute_force::removeKdigits("10", 2) == "0");
	assert(brute_force::removeKdigits("10", 1) == "0");
	assert(brute_force::removeKdigits("112", 1) == "11");
	assert(brute_force::removeKdigits("122", 1) == "12");
	assert(brute_force::removeKdigits("122", 2) == "1");
	assert(brute_force::removeKdigits("231", 2) == "1");
	assert(brute_force::removeKdigits("1234567890", 9) == "0");
}
