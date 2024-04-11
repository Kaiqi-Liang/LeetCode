/**
 * @file reveal_cards_in_increasing_order.cpp
 * @brief https://leetcode.com/problems/reveal-cards-in-increasing-order/
 * @date 2024-04-11
 */
#include <algorithm>
#include <cassert>
#include <deque>
#include <numeric>
#include <queue>
#include <set>
#include <vector>

namespace set {
	std::vector<int> deckRevealedIncreasing(std::vector<int> deck) {
		std::sort(deck.begin(), deck.end());
		std::vector<size_t> tmp(deck.size());
		std::iota(tmp.begin(), tmp.end(), 0);
		std::set<size_t> indices(tmp.cbegin(), tmp.cend());
		std::vector<int> ordering(deck.size());
		auto index_iter = indices.begin();
		auto card_iter = deck.cbegin();
		while (not indices.empty()) {
			ordering[*index_iter] = *card_iter++;
			index_iter = indices.erase(index_iter);
			if (index_iter == indices.end()) {
				index_iter = indices.begin();
				if (index_iter == indices.end()) break;
			}
			++index_iter;
			if (index_iter == indices.end()) index_iter = indices.begin();
		}
		return ordering;
	}
} // namespace set

namespace queue {
	std::vector<int> deckRevealedIncreasing(std::vector<int> deck) {
		std::sort(deck.begin(), deck.end());
		std::deque<size_t> tmp(deck.size());
		std::iota(tmp.begin(), tmp.end(), 0);
		std::queue<size_t> indices(tmp);
		std::vector<int> ordering(deck.size());
		auto card_iter = deck.cbegin();
		while (not indices.empty()) {
			ordering[indices.front()] = *card_iter++;
			indices.pop();
			indices.push(indices.front());
			indices.pop();
		}
		return ordering;
	}
} // namespace queue

int main() {
	assert(
	    set::deckRevealedIncreasing({17, 13, 11, 2, 3, 5, 7}) ==
	    std::vector<int>({2, 13, 3, 11, 5, 17, 7})
	);
	assert(
	    set::deckRevealedIncreasing({1, 1000}) == std::vector<int>({1, 1000})
	);
	assert(
	    queue::deckRevealedIncreasing({17, 13, 11, 2, 3, 5, 7}) ==
	    std::vector<int>({2, 13, 3, 11, 5, 17, 7})
	);
	assert(
	    queue::deckRevealedIncreasing({1, 1000}) == std::vector<int>({1, 1000})
	);
	return 0;
}
