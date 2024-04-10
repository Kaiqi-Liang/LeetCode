/**
 * @file serialize_and_deserialize_bst.cpp
 * @brief https://leetcode.com/problems/serialize-and-deserialize-bst/
 * @date 2024-04-10
 */
#include <cassert>
#include <queue>
#include <sstream>
#include <string>

#include "tree.hpp"
using namespace std;

class Codec {
	string serialise_dfs(TreeNode *node, string serialised) {
		if (not node) return "";
		return to_string(node->val) + ' ' +
		       serialise_dfs(node->left, serialised) + ' ' +
		       serialise_dfs(node->right, serialised);
	}

	TreeNode *deserialise_dfs(queue<int> &vals, int lo, int hi) {
		if (vals.empty() or vals.front() < lo or vals.front() > hi) {
			return nullptr;
		}
		TreeNode *root = new TreeNode(vals.front());
		vals.pop();
		root->left = deserialise_dfs(vals, lo, root->val);
		root->right = deserialise_dfs(vals, root->val, hi);
		return root;
	}

public:
	// Encodes a tree to a single string.
	string serialize(TreeNode *root) {
		return serialise_dfs(root, "");
	}

	// Decodes your encoded data to tree.
	TreeNode *deserialize(string data) {
		stringstream ss{data};
		int val;
		queue<int> vals;
		while (ss >> val) {
			vals.push(val);
		}
		return deserialise_dfs(vals, -10001, 10001);
	}
};

int main() {
	Codec ser, deser;
	{
		TreeNode nine = TreeNode(9);
		TreeNode eight = TreeNode(8, nullptr, &nine);
		TreeNode ten = TreeNode(10, &eight, nullptr);
		TreeNode five = TreeNode(5, nullptr, &ten);
		string serialised = ser.serialize(&five);
		assert(isSameTree(&five, deser.deserialize(serialised)));
	}
	{
		TreeNode three = TreeNode(3);
		TreeNode one = TreeNode(1);
		TreeNode two = TreeNode(2, &one, &three);
		TreeNode four = TreeNode(4, &two, nullptr);
		string serialised = ser.serialize(&four);
		assert(isSameTree(&four, deser.deserialize(serialised)));
	}
	return 0;
}
