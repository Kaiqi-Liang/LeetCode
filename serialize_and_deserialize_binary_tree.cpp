/**
 * @file serialize_and_deserialize_binary_tree.cpp
 * @brief https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
 * @date 2024-04-06
 */
#include <cassert>
#include <sstream>
#include <string>

#include "tree.hpp"
using namespace std;

class Codec {
	string serialise_dfs(TreeNode *node, string serialised) {
		if (not node) return serialised + "1001";
		return to_string(node->val) + ' ' +
		       serialise_dfs(node->left, serialised) + ' ' +
		       serialise_dfs(node->right, serialised);
	}

	TreeNode *get_tree_node(stringstream &ss) {
		if (ss.eof()) return nullptr;
		int val;
		ss >> val;
		if (val < 1001) return new TreeNode(val);
		return nullptr;
	}

	TreeNode *deserialise_dfs(stringstream &ss) {
		TreeNode *newNode = get_tree_node(ss);
		if (newNode) {
			newNode->left = deserialise_dfs(ss);
			newNode->right = deserialise_dfs(ss);
		}
		return newNode;
	}

public:
	// Encodes a tree to a single string.
	string serialize(TreeNode *root) {
		return serialise_dfs(root, "");
	}

	// Decodes your encoded data to tree.
	TreeNode *deserialize(string data) {
		stringstream ss{data};
		return deserialise_dfs(ss);
	}
};

int main() {
	Codec ser, deser;
	TreeNode three = TreeNode(3);
	TreeNode two = TreeNode(2);
	TreeNode one = TreeNode(1);
	one.left = &two;
	one.right = &three;
	string serialised1 = ser.serialize(&one);
	assert(isSameTree(&one, deser.deserialize(serialised1)));

	one.right = nullptr;
	two.left = &three;
	string serialised2 = ser.serialize(&one);
	assert(isSameTree(&one, deser.deserialize(serialised2)));

	assert(serialised1 != serialised2);
	return 0;
}
