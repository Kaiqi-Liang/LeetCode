/**
 * @file lowest_common_ancestor_of_a_binary_tree.cpp
 * @brief https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
 * @date 2023-03-14
 */
#include <cassert>
#include <unordered_set>
#include <vector>

#include "tree.hpp"

namespace dfs {
	bool dfs(TreeNode *src, TreeNode *dest, std::vector<TreeNode *> &path) {
		if (not src or not dest) return false;
		if (src->val == dest->val) return true;
		if (dfs(src->left, dest, path)) {
			path.push_back(src->left);
			return true;
		}
		if (dfs(src->right, dest, path)) {
			path.push_back(src->right);
			return true;
		}
		return false;
	}

	TreeNode *lowestCommonAncestor(TreeNode *root, TreeNode *p, TreeNode *q) {
		std::vector<TreeNode *> path_to_p;
		std::vector<TreeNode *> path_to_q;
		dfs(root, p, path_to_p);
		dfs(root, q, path_to_q);
		size_t i = 0;
		std::unordered_set<int> set;
		for (; i < std::min(path_to_p.size(), path_to_q.size()); i++) {
			if (set.contains(path_to_p[i]->val)) {
				return path_to_p[i];
			}
			set.insert(path_to_p[i]->val);
			if (set.contains(path_to_q[i]->val)) {
				return path_to_q[i];
			}
			set.insert(path_to_q[i]->val);
		}
		if (i == path_to_p.size()) {
			for (size_t j = i; j < path_to_q.size(); ++j) {
				if (set.contains(path_to_q[j]->val)) {
					return path_to_q[j];
				}
			}
		} else {
			for (; i < path_to_p.size(); ++i) {
				if (set.contains(path_to_p[i]->val)) {
					return path_to_p[i];
				}
			}
		}
		return root;
	}
} // namespace dfs

namespace recursion {
	TreeNode *lowestCommonAncestor(TreeNode *root, TreeNode *p, TreeNode *q) {
		if (not root) return nullptr;
		if (root->val == p->val or root->val == q->val) return root;
		TreeNode *left_lca = lowestCommonAncestor(root->left, p, q);
		TreeNode *right_lca = lowestCommonAncestor(root->right, p, q);
		if (not left_lca) return right_lca;
		if (not right_lca) return left_lca;
		return root;
	}
}

int main() {
	/*
	1
	|
	2
	*/
	TreeNode one = TreeNode(1);
	TreeNode two = TreeNode(2);
	one.left = &two;
	assert(dfs::lowestCommonAncestor(&one, &one, &two) == &one);
	assert(recursion::lowestCommonAncestor(&one, &one, &two) == &one);
	assert(lowestCommonAncestor(&one, one.val, two.val) == &one);

	TreeNode zero = TreeNode(0);
	TreeNode three = TreeNode(3);
	TreeNode four = TreeNode(4);
	TreeNode five = TreeNode(5);
	TreeNode six = TreeNode(6);
	TreeNode seven = TreeNode(7);
	TreeNode eight = TreeNode(8);
	three.left = &five;
	five.left = &six;
	five.right = &two;
	two.left = &seven;
	two.right = &four;
	three.right = &one;
	one.left = &zero;
	one.right = &eight;
	assert(dfs::lowestCommonAncestor(&three, &five, &one) == &three);
	assert(recursion::lowestCommonAncestor(&three, &five, &one) == &three);
	assert(lowestCommonAncestor(&three, five.val, one.val) == &three);

	/*
	-1
	/  \
  0    3
 / \
-2  4
|
8
	*/
	TreeNode negative_one = TreeNode(-1);
	TreeNode negative_two = TreeNode(-2);
	negative_one.left = &zero;
	negative_one.right = &three;
	zero.left = &negative_two;
	zero.right = &four;
	three.left = nullptr;
	three.right = nullptr;
	negative_two.left = &eight;
	assert(dfs::lowestCommonAncestor(&negative_one, &eight, &four) == &zero);
	assert(recursion::lowestCommonAncestor(&negative_one, &eight, &four) == &zero);
	assert(lowestCommonAncestor(&negative_one, eight.val, four.val) == &zero);
	return 0;
}
