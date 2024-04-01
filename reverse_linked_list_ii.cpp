/**
 * @file reverse_linked_list_ii.cpp
 * @brief https://leetcode.com/problems/reverse-linked-list-ii/
 * @date 2024-04-01
 */
#include <cassert>

#include "list.hpp"

ListNode *reverseBetween(ListNode *head, int left, int right) {
	if (left == right) return head;
	int i = 1;
	ListNode *tmp = nullptr;
	ListNode *prev = nullptr;
	ListNode *first = head;
	ListNode *curr = head;
	for (; curr != nullptr; curr = tmp, ++i) {
		tmp = curr->next;
		if (i < left) {
			first = curr;
		} else if (i <= right) {
			curr->next = prev;
		} else {
			break;
		}
		prev = curr;
	}
	if (left == 1) {
		if (curr != nullptr) { // right < last
			first->next = curr;
		}
		return prev;
	}
	first->next->next = curr;
	first->next = prev;
	return head;
}

int main() {
	ListNode node1 = ListNode(5);
	ListNode node2 = ListNode(4, &node1);
	ListNode node3 = ListNode(3, &node2);
	ListNode node4 = ListNode(2, &node3);
	ListNode node5 = ListNode(1, &node4);
	{
		ListNode *reversed_list = reverseBetween(&node5, 2, 4);
		assert(reversed_list->val == 1);
		assert(reversed_list->next->val == 4);
		assert(reversed_list->next->next->val == 3);
		assert(reversed_list->next->next->next->val == 2);
		assert(reversed_list->next->next->next->next->val == 5);
		assert(reversed_list->next->next->next->next->next == nullptr);
	}
	{
		ListNode *reversed_list = reverseBetween(&node1, 1, 1);
		assert(reversed_list->val == 5);
		assert(reversed_list->next == nullptr);
	}
	{
		ListNode *reversed_list = reverseBetween(&node4, 1, 1);
		assert(reversed_list->val == 2);
		assert(reversed_list->next->val == 5);
		assert(reversed_list->next->next == nullptr);
	}
	{
		// 2 -> 5
		ListNode *reversed_list = reverseBetween(&node4, 1, 2);
		// 5 -> 2
		assert(reversed_list->val == 5);
		assert(reversed_list->next->val == 2);
		assert(reversed_list->next->next == nullptr);
	}
	{
		// 4 -> 3 -> 2
		ListNode *reversed_list = reverseBetween(&node2, 1, 2);
		// 3 -> 4 -> 2
		assert(reversed_list->val == 3);
		assert(reversed_list->next->val == 4);
		assert(reversed_list->next->next->val == 2);
		assert(reversed_list->next->next->next == nullptr);
		return 0;
	}
}
