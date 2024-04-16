/**
 * @file asteroid_collision.cpp
 * @brief https://leetcode.com/problems/asteroid-collision/
 * @date 2024-04-17
 */
#include <cassert>
#include <stack>
#include <vector>
using namespace std;

vector<int> asteroidCollision(vector<int> const &asteroids) {
	stack<int> s;
	for (auto &&asteroid : asteroids) {
		while (not s.empty() and s.top() > 0 and asteroid < 0 and
		       abs(asteroid) > abs(s.top()))
		{
			s.pop();
		}
		if (s.empty() or s.top() * asteroid > 0 or
		    (s.top() < 0 and asteroid > 0))
		{
			s.push(asteroid);
		} else if (abs(asteroid) == abs(s.top())) {
			s.pop();
		}
	}
	vector<int> state;
	while (not s.empty()) {
		state.push_back(s.top());
		s.pop();
	}
	return {state.crbegin(), state.crend()};
}

int main() {
	assert(asteroidCollision({5, 10, -5}) == vector<int>({5, 10}));
	assert(asteroidCollision({10, 2, -5}) == vector<int>{10});
	assert(asteroidCollision({5, -5}) == vector<int>{});
	assert(asteroidCollision({5, 10, -20}) == vector<int>{-20});
	assert(asteroidCollision({10, 2, -10}) == vector<int>{});
	assert(asteroidCollision({-10, 5, 2}) == vector<int>({-10, 5, 2}));
	assert(asteroidCollision({-10}) == vector<int>{-10});
	assert(asteroidCollision({-10, -2}) == vector<int>({-10, -2}));
	assert(asteroidCollision({-2, -1, 1, 2}) == vector<int>({-2, -1, 1, 2}));
	return 0;
}