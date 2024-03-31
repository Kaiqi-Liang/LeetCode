/**
 * @file sort_list.cpp
 * @brief https://leetcode.com/problems/sort-list/
 * @date 2023-03-23
 */
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#include "list.hpp"
using namespace std;

namespace vec {
	ListNode *sortList(ListNode *head) {
		vector<int> vec;
		for (ListNode *curr = head; curr; curr = curr->next) {
			vec.push_back(curr->val);
		}
		sort(vec.begin(), vec.end());
		size_t i = 0;
		for (ListNode *curr = head; curr and i < vec.size();
		     curr = curr->next, ++i)
		{
			curr->val = vec[i];
		}
		return head;
	}
} // namespace vec

namespace merge {
	ListNode *merge(ListNode *list1, ListNode *list2, size_t size) {
		ListNode *curr1 = list1;
		ListNode *curr2 = list2;
		vector<int> list3;
		size_t i = 0;
		while (curr1 != list2 and i < size) {
			if (curr1->val < curr2->val) {
				list3.push_back(curr1->val);
				curr1 = curr1->next;
			} else {
				list3.push_back(curr2->val);
				curr2 = curr2->next;
				++i;
			}
		}
		while (curr1 != list2) {
			list3.push_back(curr1->val);
			curr1 = curr1->next;
		}
		while (i < size) {
			list3.push_back(curr2->val);
			curr2 = curr2->next;
			++i;
		}
		i = 0;
		for (ListNode *curr1 = list1; curr1 != list2; curr1 = curr1->next) {
			curr1->val = list3[i++];
		}
		for (ListNode *curr2 = list2; i < list3.size(); curr2 = curr2->next) {
			curr2->val = list3[i++];
		}
		return list1;
	}

	ListNode *mergeSort(ListNode *list, size_t size) {
		if (size <= 1) return list;
		ListNode *curr = list;
		ListNode *first_half = mergeSort(list, size / 2);
		for (size_t i = 0; i < size / 2; ++i) {
			curr = curr->next;
		}
		ListNode *second_half = mergeSort(curr, size / 2 + size % 2);
		return merge(first_half, second_half, size / 2 + size % 2);
	}

	ListNode *sortList(ListNode *list) {
		size_t size = 0;
		for (ListNode *curr = list; curr; curr = curr->next) {
			++size;
		}
		return mergeSort(list, size);
	}
} // namespace merge

int main() {
	{
		auto node1 = ListNode(3);
		auto node2 = ListNode(1, &node1);
		auto node3 = ListNode(2, &node2);
		auto list1 = ListNode(4, &node3);
		auto list2 = vec::sortList(&list1);
		for (ListNode *curr = list2; curr; curr = curr->next) {
			cout << curr->val << endl;
		}
	}
	cout << endl;
	{
		auto node1 = ListNode(4);
		auto node2 = ListNode(2, &node1);
		auto node3 = ListNode(4, &node2);
		auto node4 = ListNode(3, &node3);
		auto list1 = ListNode(5, &node4);
		auto list2 = merge::sortList(&list1);
		for (ListNode *curr = list2; curr; curr = curr->next) {
			cout << curr->val << endl;
		}
	}
	assert(merge::sortList(nullptr) == nullptr);
}
