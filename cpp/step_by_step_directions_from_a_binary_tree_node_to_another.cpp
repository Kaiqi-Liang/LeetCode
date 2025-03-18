/**
 * @file step_by_step_directions_from_a_binary_tree_node_to_another.cpp
 * @brief
 * https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/
 * @date 2025-03-18
 */
#include <cassert>
#include <string>
using namespace std;

#include "tree.hpp"

namespace {
	bool get_directions(TreeNode *node, int dest, string &directions) {
		if (not node) return false;
		if (node->val == dest) return true;

		directions += 'L';
		if (get_directions(node->left, dest, directions)) return true;
		directions.pop_back();

		directions += 'R';
		if (get_directions(node->right, dest, directions)) return true;
		directions.pop_back();

		return false;
	}
} // namespace

string getDirections(TreeNode *root, int startValue, int destValue) {
	TreeNode *lowest_common_ancestor =
	    lowestCommonAncestor(root, startValue, destValue);
	string directions;
	for (TreeNode *node : get_path(root, startValue)) {
		if (node == lowest_common_ancestor) {
			get_directions(node, destValue, directions);
			return directions;
		} else {
			directions += 'U';
		}
	}
	throw runtime_error("No path between start and dest");
}

int main() {
	{
		TreeNode one(1);
		TreeNode two(2, &one, nullptr);
		assert(getDirections(&two, 2, 1) == "L");
	}
	{
		TreeNode three(3);
		TreeNode six(6);
		TreeNode four(4);
		TreeNode one(1, &three, nullptr);
		TreeNode two(2, &six, &four);
		TreeNode five(5, &one, &two);
		assert(getDirections(&five, 3, 6) == "UURL");
	}
	return 0;
}
