/**
 * @file smallest_string_starting_from_leaf.cpp
 * @brief https://leetcode.com/problems/smallest-string-starting-from-leaf/
 * @date 2024-04-18
 */
#include <algorithm>
#include <cassert>
#include <string>
#include <unordered_set>

#include "tree.hpp"
using namespace std;

int char_to_int(char val) {
	return val - 'a';
}

string int_to_string(int val) {
	return string(1, val + 'a');
}

void smallest_from_leaf(TreeNode *root, unordered_set<string> &strings, string curr) {
	if (root == nullptr) return;
	if (root->left == nullptr and root->right == nullptr) {
		curr += int_to_string(root->val);
		reverse(curr.begin(), curr.end());
		strings.insert(curr);
		return;
	}
	smallest_from_leaf(root->left, strings, curr + int_to_string(root->val));
	smallest_from_leaf(root->right, strings, curr + int_to_string(root->val));
}

string smallestFromLeaf(TreeNode *root) {
	unordered_set<string> strings;
	smallest_from_leaf(root, strings, "");
	return *min_element(strings.cbegin(), strings.cend());
}

int main() {
	TreeNode d1(char_to_int('d'));
	TreeNode e1(char_to_int('e'));
	TreeNode b(char_to_int('b'), &d1, &e1);
	TreeNode d2(char_to_int('d'));
	TreeNode e2(char_to_int('e'));
	TreeNode c(char_to_int('c'), &d2, &e2);
	TreeNode a(char_to_int('a'), &b, &c);
	assert(smallestFromLeaf(&a) == "dba");
	return 0;
}
