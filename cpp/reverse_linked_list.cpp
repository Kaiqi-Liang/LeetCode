/**
 * @file reverse_linked_list.cpp
 * @brief https://leetcode.com/problems/reverse-linked-list/
 * @date 2024-04-01
 */
#include <cassert>

#include "list.hpp"

namespace iterative {
	ListNode *reverseList(ListNode *head) {
		ListNode *list = nullptr;
		for (ListNode *curr = head, *curr2 = list; curr != nullptr;
		     curr = curr->next, curr2 = list)
		{
			ListNode *newNode = new ListNode(curr->val);
			list = newNode;
			list->next = curr2;
		}
		return list;
	}
} // namespace iterative

namespace recursive {
	ListNode *recurseList(ListNode *newList, ListNode *originalList) {
		if (originalList == nullptr) return newList;
		newList = new ListNode(originalList->val, newList);
		return recurseList(newList, originalList->next);
	}

	ListNode *reverseList(ListNode *head) {
		return recurseList(nullptr, head);
	}
} // namespace recursive

int main() {
	auto node1 = ListNode(4);
	auto node2 = ListNode(2, &node1);
	auto list1 = ListNode(1, &node2);
	auto list2 = reverseList(&list1);
	assert(list2->val == 4);
	assert(list2->next->val == 2);
	assert(list2->next->next->val == 1);
	assert(list2->next->next->next == nullptr);
	return 0;
}
