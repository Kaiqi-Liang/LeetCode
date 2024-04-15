/**
 * @file sum_root_to_leaf_numbers.cpp
 * @brief https://leetcode.com/problems/sum-root-to-leaf-numbers/
 * @date 2024-04-15
 */
#include <cassert>

#include "tree.hpp"

void sum_numbers(TreeNode *root, int &sum, int curr) {
	if (root == nullptr) return;
	int num = curr * 10 + root->val;
	if (root->left == nullptr and root->right == nullptr) {
		sum += num;
		return;
	}
	sum_numbers(root->left, sum, num);
	sum_numbers(root->right, sum, num);
}

int sumNumbers(TreeNode *root) {
	int res = 0;
	sum_numbers(root, res, 0);
	return res;
}

int main() {
	TreeNode five(5);
	TreeNode nine(9, &five, nullptr);
	TreeNode zero(0);
	TreeNode four(4, &nine, &zero);
	assert(sumNumbers(&four) == 535);
	return 0;
}
