/**
 * https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/ 
 * @date 2022-06-23
 *
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */
interface TreeNode {
    val: number;
    left: TreeNode | null;
    right: TreeNode | null;
}

function sortedArrayToBST(nums: number[]): TreeNode | null {
    let tree: TreeNode | null = null;
    if (nums.length) {
        const midpoint = Math.round(nums.length / 2) - 1;
        tree = {
            val: nums[midpoint],
            left: sortedArrayToBST(nums.slice(0, midpoint)),
            right: sortedArrayToBST(nums.slice(midpoint + 1)),
        };
    }
    return tree;
}

console.log(sortedArrayToBST([1, 2, 3, 4]));
