/**
 * @file sum_of_left_leaves.cpp
 * @brief https://leetcode.com/problems/sum-of-left-leaves/
 * @date 2024-04-14
 */
#include <cassert>

#include "tree.hpp"

int sum_of_left_leaves(TreeNode *root, bool is_left) {
	if (root == nullptr) return 0;
	if (root->left == nullptr and root->right == nullptr) {
		return is_left ? root->val : 0;
	}
	return sum_of_left_leaves(root->left, true) +
	       sum_of_left_leaves(root->right, false);
}

int sumOfLeftLeaves(TreeNode *root) {
	return sum_of_left_leaves(root, false);
}

int main() {
	TreeNode two(2);
	TreeNode one(1, &two, nullptr);
	assert(sumOfLeftLeaves(&one) == 2);
	one.left = nullptr;
	one.right = &two;
	assert(sumOfLeftLeaves(&one) == 0);
	return 0;
}
