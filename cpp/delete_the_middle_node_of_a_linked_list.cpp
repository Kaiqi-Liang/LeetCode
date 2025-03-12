/**
 * @file delete_the_middle_node_of_a_linked_list.cpp
 * @brief https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/
 * @date 2025-03-12
 */
#include <cassert>
#include <cstddef>

#include "list.hpp"

ListNode *deleteMiddle(ListNode *head) {
	if (not head->next) {
		return nullptr;
	}
	size_t count = 0;
	for (ListNode *curr = head; curr != nullptr; curr = curr->next) {
		++count;
	}
	ListNode *curr = head;
	for (size_t i = 0; i < count / 2 - 1; ++i) {
		curr = curr->next;
	}
	curr->next = curr->next->next;
	return head;
}

int main() {
	ListNode one(1);
	assert(deleteMiddle(&one) == nullptr);
	ListNode two(2, &one);
	assert(deleteMiddle(&two) == &two);
	assert(two.next == nullptr);
}
