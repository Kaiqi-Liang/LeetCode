/**
 * @file n_ary_tree_postorder_traversal.cpp
 * @brief https://leetcode.com/problems/n-ary-tree-postorder-traversal/
 * @date 2024-08-27
 */
#include <cassert>
#include <iterator>
#include <numeric>
#include <vector>

#include "node.hpp"
using namespace std;

vector<int> postorder(Node *root) {
	if (root == nullptr) return {};
	vector<int> nodes = accumulate(
	    root->children.cbegin(),
	    root->children.cend(),
	    vector<int>{},
	    [](vector<int> a, Node *c) {
		    vector<int> nodes = postorder(c);
		    a.insert(a.end(), nodes.begin(), nodes.end());
		    return a;
	    }
	);
	nodes.push_back(root->val);
	return nodes;
}

int main() {
	Node two(2);
	Node four(4);
	Node five(5);
	Node six(6);
	Node three(3, {&five, &six});
	Node one(1, {&three, &two, &four});
	assert(postorder(&one) == vector<int>({5, 6, 3, 2, 4, 1}));
}
