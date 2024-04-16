/**
 * @file add_one_row_to_tree.cpp
 * @brief https://leetcode.com/problems/add-one-row-to-tree/
 * @date 2024-04-17
 */
#include <cassert>

#include "tree.hpp"

void find_depth(TreeNode *root, int val, int depth, int curr_depth) {
	if (root == nullptr) {
		return;
	}
	if (curr_depth == depth - 1) {
		root->left = new TreeNode(val, root->left, nullptr);
		root->right = new TreeNode(val, nullptr, root->right);
		return;
	}
	find_depth(root->left, val, depth, curr_depth + 1);
	find_depth(root->right, val, depth, curr_depth + 1);
}

TreeNode *addOneRow(TreeNode *root, int val, int depth) {
	if (depth == 1) {
		root = new TreeNode(val, root, nullptr);
	} else {
		find_depth(root, val, depth, 1);
	}
	return root;
}

int main() {
	TreeNode node(1);
	addOneRow(&node, 2, 2);
	assert(node.left->val == 2);
	assert(node.left->left == nullptr);
	assert(node.left->right == nullptr);
	assert(node.right->val == 2);
	assert(node.right->left == nullptr);
	assert(node.right->right == nullptr);

	TreeNode *root = addOneRow(&node, 0, 1);
	assert(root->val == 0);
	assert(root->left->val == 1);
	assert(root->right == nullptr);
	return 0;
}
