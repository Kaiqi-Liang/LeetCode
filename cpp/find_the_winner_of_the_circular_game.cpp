/**
 * @file find_the_winner_of_the_circular_game.cpp
 * @brief https://leetcode.com/problems/find-the-winner-of-the-circular-game/
 * @date 2024-07-08
 */
#include <cassert>
#include <cstddef>

#include "list.hpp"

int findTheWinner(int n, int k) {
	ListNode *circle = new ListNode(1);
	ListNode *curr = circle;
	for (int i = 1; i <= n; i++) {
		ListNode *new_node = i == n ? circle : new ListNode(i + 1);
		curr->next = new_node;
		curr = curr->next;
	}
	while (curr->next != curr) {
		for (int i = 1; i < (k == 1 ? n : k - 1); ++i) {
			curr = curr->next;
		}
		ListNode *tmp = curr->next;
		curr->next = curr->next->next;
		curr = curr->next;
		delete tmp;
		--n;
	}
	return curr->val;
}

int main() {
	assert(findTheWinner(5, 2) == 3);
	assert(findTheWinner(6, 5) == 1);
	assert(findTheWinner(6, 1) == 6);
	return 0;
}
