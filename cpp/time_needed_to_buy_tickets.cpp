/**
 * @file time_needed_to_buy_tickets.cpp
 * @brief https://leetcode.com/problems/time-needed-to-buy-tickets/
 * @date 2024-04-11
 */
#include <cassert>
#include <deque>
#include <queue>
#include <vector>
using namespace std;

namespace simulation {
	int timeRequiredToBuy(vector<size_t> const &tickets, int k) {
		deque<size_t> tmp(tickets.cbegin(), tickets.cend());
		queue<size_t> q(tmp);
		int time = 0;
		while (true) {
			++time;
			if (q.front() > 1) {
				q.push(q.front() - 1);
			} else if (k == 0) {
				break;
			}
			q.pop();
			if (--k < 0) {
				k = q.size() - 1;
			}
		}
		return time;
	}
} // namespace simulation

namespace linear {
	size_t timeRequiredToBuy(vector<size_t> tickets, size_t k) {
		size_t time = 0;
		for (size_t i = 0; i < tickets.size(); ++i) {
			time += min(tickets[k], tickets[i]);
			if (i == k) {
				--tickets[k];
			}
		}
		return time;
	}
} // namespace linear

int main() {
	assert(simulation::timeRequiredToBuy({2, 3, 2}, 2) == 6);
	assert(simulation::timeRequiredToBuy({5, 1, 1, 1}, 0) == 8);
	assert(linear::timeRequiredToBuy({2, 3, 2}, 2) == 6);
	assert(linear::timeRequiredToBuy({5, 1, 1, 1}, 0) == 8);
	return 0;
}
