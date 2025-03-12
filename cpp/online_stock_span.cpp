/**
 * @file online_stock_span.cpp
 * @brief https://leetcode.com/problems/online-stock-span/
 * @date 2025-03-12
 */
#include <cassert>
#include <stack>
using namespace std;

class StockSpanner {
public:
	int next(int price) {
		size_t span = 1;
		while (not monotonic_stack_.empty() and
		       price >= monotonic_stack_.top().first)
		{
			span += monotonic_stack_.top().second;
			monotonic_stack_.pop();
		}
		monotonic_stack_.emplace(price, span);
		return span;
	}

private:
	stack<pair<int, size_t>> monotonic_stack_;
};

int main() {
	StockSpanner stockSpanner = StockSpanner();
	assert(stockSpanner.next(100) == 1);
	assert(stockSpanner.next(80) == 1);
	assert(stockSpanner.next(60) == 1);
	assert(stockSpanner.next(70) == 2);
	assert(stockSpanner.next(60) == 1);
	assert(stockSpanner.next(75) == 4);
	assert(stockSpanner.next(85) == 6);
	return 0;
}
