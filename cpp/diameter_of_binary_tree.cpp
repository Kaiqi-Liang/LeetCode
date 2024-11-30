/**
 * @file diameter_of_binary_tree.cpp
 * @brief https://leetcode.com/problems/diameter-of-binary-tree/
 * @date 2024-11-30
 */
#include <algorithm>
#include <cassert>
using namespace std;

#include "tree.hpp"
namespace {
	int maxBranchLength(TreeNode *root, int &diameter) {
		if (not root) return 0;
		int left = maxBranchLength(root->left, diameter);
		int right = maxBranchLength(root->right, diameter);
		diameter = max(diameter, left + right);
		return max(left, right) + 1;
	}
} // namespace

int diameterOfBinaryTree(TreeNode *root) {
	int diameter = 0;
	maxBranchLength(root, diameter);
	return diameter;
}

int main() {
	TreeNode five(5);
	TreeNode four(4);
	TreeNode three(3);
	TreeNode two(2, &four, &five);
	TreeNode one(1, &two, &three);
	assert(diameterOfBinaryTree(&one) == 3);
}
