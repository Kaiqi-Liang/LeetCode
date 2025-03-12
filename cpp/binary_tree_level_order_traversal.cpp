/**
 * @file binary_tree_level_order_traversal.cpp
 * @brief https://leetcode.com/problems/binary-tree-level-order-traversal/
 * @date 2025-03-12
 */
#include <cassert>
#include <cstddef>
#include <queue>
#include <vector>
using namespace std;

#include "tree.hpp"

vector<vector<int>> levelOrder(TreeNode *root) {
	if (not root) return {};
	queue<pair<TreeNode *, size_t>> q{{{root, 0}}};
	vector<vector<int>> nodes{{root->val}};
	while (not q.empty()) {
		auto [curr, depth] = q.front();
		q.pop();
		if (not curr->left and not curr->right) continue;
		++depth;
		if (nodes.size() <= depth) {
			nodes.push_back({});
		}
		if (curr->left) {
			q.emplace(curr->left, depth);
			nodes[depth].push_back(curr->left->val);
		}
		if (curr->right) {
			q.emplace(curr->right, depth);
			nodes[depth].push_back(curr->right->val);
		}
	}
	return nodes;
}

int main() {
	TreeNode nine(9);
	TreeNode fifteen(15);
	TreeNode seven(7);
	TreeNode twenty(20, &fifteen, &seven);
	TreeNode three(3, &nine, &twenty);
	assert(
	    levelOrder(&three) == vector({vector{3}, vector{9, 20}, vector{15, 7}})
	);
	TreeNode root(1);
	assert(levelOrder(&root) == vector({vector{1}}));
	assert(levelOrder(nullptr) == vector<vector<int>>());
	return 0;
}
