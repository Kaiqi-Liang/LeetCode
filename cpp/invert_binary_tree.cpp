/**
 * @file same_tree.cpp
 * @brief https://leetcode.com/problems/symmetric-tree/
 * @date 2024-03-23
 */
#include <algorithm>
#include <cassert>

#include "tree.hpp"

TreeNode *invertTree(TreeNode *root) {
	if (root == nullptr) return nullptr;
	std::swap(root->left, root->right);
	invertTree(root->left);
	invertTree(root->right);
	return root;
}

int main() {
	TreeNode three = TreeNode(3);
	TreeNode two = TreeNode(2);
	TreeNode one = TreeNode(1, &two, &three);
	invertTree(&one);
	assert(one.val == 1);
	assert(one.left->val == 3);
	assert(one.right->val == 2);
	return 0;
}
