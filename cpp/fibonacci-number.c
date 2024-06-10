/**
 * @file fibonacci-number.c
 * @brief https://leetcode.com/problems/fibonacci-number/
 * @date 2022-04-30
 */
#include <stdlib.h>
#include <assert.h>

int rec(int n, int *opt) {
	if (n == 1) return 1;
	int first = opt[n - 1] != -1 ? opt[n - 1] : rec(n - 1, opt);
	int second = opt[n - 2] != -1 ? opt[n - 2] : rec(n - 2, opt);
	int sum = first + second;
	return opt[n] = sum;
}
int fib_recursive(int n) {
	if (n < 1) return 0;
	int *opt = malloc((n + 1) * sizeof(int));
	for (size_t i = 0; i <= n; ++i) {
		opt[i] = -1;
	}
	opt[0] = 0;
	opt[1] = 1;
	int res = rec(n, opt);
	free(opt);
	return res;
}

int fib_iterative(int n) {
	if (n < 1) return 0;
	int first = 0;
	int second = 1;
	for (int i = 2; i < n; ++i) {
		int tmp = first;
		first = second;
		second += tmp;
	}
	return first + second;
}

int main() {
	assert(fib_iterative(0) == 0);
	assert(fib_iterative(1) == 1);
	assert(fib_iterative(2) == 1);
	assert(fib_iterative(3) == 2);
	assert(fib_iterative(4) == 3);
	assert(fib_iterative(5) == 5);
	assert(fib_iterative(6) == 8);
	assert(fib_iterative(7) == 13);
	assert(fib_recursive(0) == 0);
	assert(fib_recursive(1) == 1);
	assert(fib_recursive(2) == 1);
	assert(fib_recursive(3) == 2);
	assert(fib_recursive(4) == 3);
	assert(fib_recursive(5) == 5);
	assert(fib_recursive(6) == 8);
	assert(fib_recursive(7) == 13);
	return 0;
}
