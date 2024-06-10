/**
 * @file add_two_numbers.cpp
 * @brief https://leetcode.com/problems/add-two-numbers/
 * @date 2024-04-01
 */
#include <cassert>

#include "list.hpp"
ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
	bool carry = false;
	ListNode *l3 = nullptr, *curr1 = l1, *curr2 = l2, *curr3 = l3;
	for (; curr1 != nullptr and curr2 != nullptr;
	     curr1 = curr1->next, curr2 = curr2->next)
	{
		int sum = curr1->val + curr2->val + carry;
		carry = false;
		if (sum >= 10) {
			sum -= 10;
			carry = true;
		}
		curr3 = insert(&l3, curr3, sum);
	}
	while (curr1 != nullptr) {
		int sum = curr1->val + carry;
		carry = false;
		if (sum >= 10) {
			sum -= 10;
			carry = true;
		}
		curr3 = insert(&l3, curr3, sum);
		curr1 = curr1->next;
	}
	while (curr2 != nullptr) {
		int sum = curr2->val + carry;
		carry = false;
		if (sum >= 10) {
			sum -= 10;
			carry = true;
		}
		curr3 = insert(&l3, curr3, sum);
		curr2 = curr2->next;
	}
	if (carry) {
		insert(&l3, curr3, carry);
	}
	return l3;
}

int main() {
	{
		ListNode node1 = ListNode(3);
		ListNode node2 = ListNode(4, &node1);
		ListNode list1 = ListNode(2, &node2);
		ListNode node3 = ListNode(4);
		ListNode node4 = ListNode(6, &node3);
		ListNode list2 = ListNode(5, &node4);
		ListNode *list3 = addTwoNumbers(&list1, &list2);
		assert(list3->val == 7);
		assert(list3->next->val == 0);
		assert(list3->next->next->val == 8);
		assert(list3->next->next->next == nullptr);
	}
	{
		ListNode node1 = ListNode(7);
		ListNode node2 = ListNode(2, &node1);
		ListNode list1 = ListNode(8, &node2);
		ListNode node3 = ListNode(4);
		ListNode node4 = ListNode(3, &node3);
		ListNode list2 = ListNode(1, &node4);
		ListNode *list3 = addTwoNumbers(&list1, &list2);
		assert(list3->val == 9);
		assert(list3->next->val == 5);
		assert(list3->next->next->val == 1);
		assert(list3->next->next->next->val == 1);
		assert(list3->next->next->next->next == nullptr);
		ListNode *list4 = addTwoNumbers(list3, &list1);
		assert(list4->val == 7);
		assert(list4->next->val == 8);
		assert(list4->next->next->val == 8);
		assert(list4->next->next->next->val == 1);
		assert(list4->next->next->next->next == nullptr);
	}
	return 0;
}
