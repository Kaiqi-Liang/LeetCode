/**
 * @file same_tree.cpp
 * @brief https://leetcode.com/problems/same-tree/
 * @date 2024-03-23
 */
#include <cassert>

#include "tree.hpp"

int main() {
	TreeNode two = TreeNode(2);
	TreeNode three = TreeNode(3);
	TreeNode one = TreeNode(1, &two, &three);
	assert(isSameTree(&one, &one));
	assert(not isSameTree(&one, &two));
	assert(not isSameTree(&two, &three));
	assert(isSameTree(nullptr, nullptr));
	assert(not isSameTree(&one, nullptr));
	TreeNode two_copy = TreeNode(2);
	TreeNode three_copy = TreeNode(3);
	TreeNode one_copy = TreeNode(1, &two_copy, &three_copy);
	assert(isSameTree(&one, &one_copy));
	return 0;
}
