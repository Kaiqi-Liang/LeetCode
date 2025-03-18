#pragma once
#include <cassert>
#include <unordered_set>
#include <vector>
using namespace std;

struct TreeNode {
	int val;
	TreeNode *left;
	TreeNode *right;
	TreeNode()
	: val(0)
	, left(nullptr)
	, right(nullptr) {}
	TreeNode(int x)
	: val(x)
	, left(nullptr)
	, right(nullptr) {}
	TreeNode(int x, TreeNode *left, TreeNode *right)
	: val(x)
	, left(left)
	, right(right) {}
};

bool isSameTree(TreeNode *p, TreeNode *q) {
	if (p == nullptr and q == nullptr)
		return true;
	else if (p == nullptr or q == nullptr)
		return false;
	if (p->val != q->val)
		return false;
	else
		return isSameTree(p->left, q->left) and isSameTree(p->right, q->right);
}

vector<TreeNode *> get_path(TreeNode *node, int dest) {
	if (not node) return {};
	if (node->val == dest) return {node};
	for (TreeNode *child : {node->left, node->right}) {
		vector<TreeNode *> path = get_path(child, dest);
		if (path.size()) {
			path.push_back(node);
			return path;
		}
	}
	return {};
}

TreeNode *lowestCommonAncestor(TreeNode *root, int p, int q) {
	vector<TreeNode *> path_to_start = get_path(root, p);
	vector<TreeNode *> path_to_dest = get_path(root, q);
	assert(path_to_start.front()->val == p);
	assert(path_to_dest.front()->val == q);
	unordered_set<int> nodes_in_path_to_dest;
	transform(
	    path_to_dest.cbegin(),
	    path_to_dest.cend(),
	    inserter(nodes_in_path_to_dest, nodes_in_path_to_dest.end()),
	    [](TreeNode *node) { return node->val; }
	);
	for (TreeNode *node : path_to_start) {
		if (nodes_in_path_to_dest.contains(node->val)) {
			return node;
		}
	}
	return nullptr;
}
