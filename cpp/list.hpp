#pragma once
struct ListNode {
	int val;
	ListNode *next;
	ListNode()
	: val(0)
	, next(nullptr) {}
	ListNode(int x)
	: val(x)
	, next(nullptr) {}
	ListNode(int x, ListNode *next)
	: val(x)
	, next(next) {}
};

inline ListNode *insert(ListNode **list, ListNode *curr, int x) {
	ListNode *newNode = new ListNode(x);
	if (*list == nullptr) {
		return *list = newNode;
	} else {
		curr->next = newNode;
		curr = curr->next;
		return curr;
	}
}

inline ListNode *reverseList(ListNode *head) {
	ListNode *tmp = nullptr;
	ListNode *prev = nullptr;
	for (ListNode *curr = head; curr != nullptr; curr = tmp) {
		tmp = curr->next;
		curr->next = prev;
		prev = curr;
	}
	return prev;
}
