/**
 * @file path_sum.cpp
 * @brief https://leetcode.com/problems/path-sum/
 * @date 2024-11-24
 */
#include <cassert>

#include "tree.hpp"
bool calculate_sum(TreeNode *curr, int target_sum, int running_sum) {
	if (not curr) {
		return false;
	}
	running_sum += curr->val;
	if (not curr->left and not curr->right) {
		// leaf
		return running_sum == target_sum;
	}
	// not a leaf
	return calculate_sum(curr->left, target_sum, running_sum) ||
	       calculate_sum(curr->right, target_sum, running_sum);
}

bool hasPathSum(TreeNode *root, int target_sum) {
	return calculate_sum(root, target_sum, 0);
}

int main() {
	TreeNode four(4);
	TreeNode three(3);
	TreeNode one(1, &four, nullptr);
	TreeNode two(2, &one, &three);
	assert(hasPathSum(&two, 5));
	return 0;
}
