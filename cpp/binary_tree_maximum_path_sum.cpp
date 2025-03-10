/**
 * @file binary_tree_maximum_path_sum.cpp
 * @brief https://leetcode.com/problems/binary-tree-maximum-path-sum/
 * @date 2025-03-10
 */

#include <algorithm>
#include <cassert>
using namespace std;

#include "tree.hpp"
namespace {
	int max_path_sum_ending_at(TreeNode *root, int &max_sum) {
		if (not root) return 0;
		int left = max(0, max_path_sum_ending_at(root->left, max_sum));
		int right = max(0, max_path_sum_ending_at(root->right, max_sum));
		max_sum = max(max_sum, left + right + root->val);
		return max(left, right) + root->val;
	}
} // namespace

int maxPathSum(TreeNode *root) {
	int max_sum = INT32_MIN;
	max_path_sum_ending_at(root, max_sum);
	return max_sum;
}

int main() {
	TreeNode fifty(15);
	TreeNode seven(7);
	TreeNode twenty(20, &fifty, &seven);
	TreeNode nine(9);
	TreeNode root(-10, &nine, &twenty);
	assert(maxPathSum(&root) == 42);
	return 0;
}
