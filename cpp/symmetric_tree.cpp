/**
 * @file same_tree.cpp
 * @brief https://leetcode.com/problems/symmetric-tree/
 * @date 2024-03-23
 */
#include <cassert>

#include "tree.hpp"

bool isMirror(TreeNode *left, TreeNode *right) {
	if (left == nullptr and right == nullptr)
		return true;
	else if (left == nullptr or right == nullptr)
		return false;
	if (left->val != right->val) return false;
	return isMirror(left->left, right->right) and
	       isMirror(left->right, right->left);
}

bool isSymmetric(TreeNode *root) {
	return isMirror(root->left, root->right);
}

int main() {
	{
		TreeNode left_three = TreeNode(3);
		TreeNode left_two = TreeNode(2, &left_three, nullptr);
		TreeNode right_three = TreeNode(3);
		TreeNode right_two = TreeNode(2, &right_three, nullptr);
		TreeNode one = TreeNode(1, &left_two, &right_two);
		assert(not isSymmetric(&one));
	}
	{
		TreeNode left_three = TreeNode(3);
		TreeNode left_two = TreeNode(2, &left_three, nullptr);
		TreeNode right_three = TreeNode(3);
		TreeNode right_two = TreeNode(2, nullptr, &right_three);
		TreeNode one = TreeNode(1, &left_two, &right_two);
		assert(isSymmetric(&one));
	}
	return 0;
}
