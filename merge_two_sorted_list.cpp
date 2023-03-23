/**
 * @file merge_two_sorted_list.cpp
 * @brief https://leetcode.com/problems/merge-two-sorted-lists/
 * @date 2023-03-23
 */
#include <iostream>

// Definition for singly-linked list.
struct ListNode {
	int val;
	ListNode* next;
	ListNode()
	: val(0)
	, next(nullptr) {}
	ListNode(int x)
	: val(x)
	, next(nullptr) {}
	ListNode(int x, ListNode* next)
	: val(x)
	, next(next) {}
};

ListNode* insert(ListNode* list, ListNode* curr, int x) {
	ListNode* newNode = new ListNode(x);
	if (not list) return newNode;
	curr->next = newNode;
	return list;
}

ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
	ListNode* curr1 = list1;
	ListNode* curr2 = list2;
	ListNode* list3 = nullptr;
	ListNode* curr3 = list3;
	while (curr1 and curr2) {
		if (curr1->val > curr2->val) {
			list3 = insert(list3, curr3, curr2->val);
			curr3 = curr3->next;
			curr2 = curr2->next;
		} else {
			list3 = insert(list3, curr3, curr1->val);
			curr3 = curr3->next;
			curr1 = curr1->next;
		}
	}
	while (curr1) {
		list3 = insert(list3, curr3, curr1->val);
		curr3 = curr3->next;
		curr1 = curr1->next;
	}
	while (curr2) {
		list3 = insert(list3, curr3, curr2->val);
		curr3 = curr3->next;
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
	ListNode* curr = list3;
	while (curr) {
		std::cout << curr->val << '\n';
		curr = curr->next;
	}
}
