/**
 * @file subtree_of_another_tree.cpp
 * @brief https://leetcode.com/problems/subtree-of-another-tree/
 * @date 2024-11-24
 */
#include <cassert>
#include "tree.hpp"
bool isSubtree(TreeNode *root, TreeNode *subRoot) {
	if (root == nullptr and subRoot == nullptr) {
		return true;
	} else if (root == nullptr or subRoot == nullptr) {
		return false;
	} else if (isSameTree(root, subRoot) or isSubtree(root->left, subRoot) or
	           isSubtree(root->right, subRoot))
	{
		return true;
	}
	return false;
}

int main() {
	TreeNode two(2);
	TreeNode one(1, &two, nullptr);
	assert(isSubtree(&one, &two));
	TreeNode subtree(1);
	assert(!isSubtree(&one, &subtree));
	
	TreeNode four(4, nullptr, &two);
	TreeNode five(5);
	TreeNode three(3, &four, & five);
	TreeNode root(3);
	assert(!isSubtree(&three, &root));
	return 0;
}
