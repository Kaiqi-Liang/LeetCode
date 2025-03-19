/**
 * @file maximum_width_of_binary_tree.cpp
 * @brief https://leetcode.com/problems/maximum-width-of-binary-tree/
 * @date 2025-03-19
 */
#include <cassert>
#include <queue>
using namespace std;

#include "tree.hpp"

int widthOfBinaryTree(TreeNode *root) {
	queue<pair<TreeNode *, size_t>> first_queue{{{root, 1}}};
	queue<pair<TreeNode *, size_t>> second_queue;
	bool curr_first = true;
	size_t max_width = 1;
	auto const &curr_queue = [&first_queue, &second_queue, &curr_first]() {
		return curr_first ? &first_queue : &second_queue;
	};
	auto const &other_queue = [&first_queue, &second_queue, &curr_first]() {
		return curr_first ? &second_queue : &first_queue;
	};
	while (true) {
		auto [curr_node, curr_index] = curr_queue()->front();
		curr_queue()->pop();
		if (curr_node->left)
			other_queue()->emplace(curr_node->left, curr_index * 2);
		if (curr_node->right)
			other_queue()->emplace(curr_node->right, curr_index * 2 + 1);
		if (curr_queue()->empty()) {
			if (not other_queue()->empty()) {
				max_width =
				    max(max_width,
				        other_queue()->back().second -
				            other_queue()->front().second + 1);
			} else {
				break;
			}
			curr_first = not curr_first;
		}
	}
	return static_cast<int>(max_width);
}

int main() {
	{
		TreeNode five(5);
		TreeNode leaf(3);
		TreeNode three(3, &five, &leaf);
		TreeNode nine(9);
		TreeNode two(2, nullptr, &nine);
		TreeNode root(1, &three, &two);
		assert(widthOfBinaryTree(&root) == 4);
	}
	{
		TreeNode five(5);
		TreeNode two(2);
		TreeNode three(3, &five, nullptr);
		TreeNode one(1, &three, &two);
		assert(widthOfBinaryTree(&one) == 2);
	}
	return 0;
}
