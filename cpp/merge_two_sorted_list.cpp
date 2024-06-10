/**
 * @file merge_two_sorted_list.cpp
 * @brief https://leetcode.com/problems/merge-two-sorted-lists/
 * @date 2023-03-23
 */
#include <iostream>

#include "list.hpp"
using namespace std;

ListNode *mergeTwoLists(ListNode *list1, ListNode *list2) {
	ListNode *curr1 = list1;
	ListNode *curr2 = list2;
	ListNode *list3 = nullptr;
	ListNode *curr3 = list3;
	while (curr1 and curr2) {
		if (curr1->val < curr2->val) {
			curr3 = insert(&list3, curr3, curr1->val);
			curr1 = curr1->next;
		} else {
			curr3 = insert(&list3, curr3, curr2->val);
			curr2 = curr2->next;
		}
	}
	while (curr1) {
		curr3 = insert(&list3, curr3, curr1->val);
		curr1 = curr1->next;
	}
	while (curr2) {
		curr3 = insert(&list3, curr3, curr2->val);
		curr2 = curr2->next;
	}
	return list3;
}

int main() {
	auto node1 = ListNode(4);
	auto node2 = ListNode(2, &node1);
	auto list1 = ListNode(1, &node2);
	auto node3 = ListNode(4);
	auto node4 = ListNode(3, &node3);
	auto list2 = ListNode(1, &node4);
	auto list3 = mergeTwoLists(&list1, &list2);
	ListNode *curr = list3;
	while (curr) {
		cout << curr->val << '\n';
		curr = curr->next;
	}
	return 0;
}
