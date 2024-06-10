/// https://leetcode.com/problems/delete-leaves-with-a-given-value/
func removeLeafNodes(_ root: TreeNode?, _ target: Int) -> TreeNode? {
    guard let curr = root else {
        return root
    }
    curr.left = removeLeafNodes(curr.left, target)
    curr.right = removeLeafNodes(curr.right, target)
    if curr.left == nil && curr.right == nil && curr.val == target {
        return nil
    }
    return root
}

func testDeleteLeavesWithAGivenValue() {
    assert(removeLeafNodes(TreeNode(1, TreeNode(2, TreeNode(2), nil), TreeNode(3, TreeNode(2), TreeNode(4))), 2) == TreeNode(1, nil, TreeNode(3, nil, TreeNode(4))))
    assert(removeLeafNodes(TreeNode(1, TreeNode(3, TreeNode(3), TreeNode(2)), TreeNode(3)), 3) == TreeNode(1, TreeNode(3, nil, TreeNode(2)), nil))
}
