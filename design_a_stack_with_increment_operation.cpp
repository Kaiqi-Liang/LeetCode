/**
 * @file design_a_stack_with_increment_operation.cpp
 * @brief https://leetcode.com/problems/design-a-stack-with-increment-operation/
 * @date 2022-02-12
 */
#include <cassert>
#include <stack>

/**
 * Your CustomStack object will be instantiated and called as such:
 * CustomStack* obj = new CustomStack(maxSize);
 * obj->push(x);
 * int param_2 = obj->pop();
 * obj->increment(k,val);
 */
class CustomStack {
public:
	CustomStack(int maxSize)
	: _stack{std::stack<int>{}}
	, _maxSize{maxSize} {}

	void push(int x) {
		if (_stack.size() < _maxSize) _stack.push(x);
	}

	int pop() {
		if (_stack.empty()) return -1;
		auto const top = _stack.top();
		_stack.pop();
		return top;
	}

	void increment(int k, int val) {
		auto stack = std::stack<int>{};
		while (not _stack.empty()) {
			auto top = _stack.top();
			if (k >= _stack.size()) top += val;
			stack.push(top);
			_stack.pop();
		}
		while (not stack.empty()) {
			_stack.push(stack.top());
			stack.pop();
		}
	}

private:
	std::stack<int> _stack;
	int _maxSize;
};

auto main() -> int {
	CustomStack *obj = new CustomStack(0);
	obj->push(1);
	int top = obj->pop();
	assert(top == -1);
	return 0;
}
