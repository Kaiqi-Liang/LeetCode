/// https://leetcode.com/problems/same-tree/
func isSameTree(_ lhs: TreeNode?, _ rhs: TreeNode?) -> Bool {
    lhs == rhs
}

func testSameTree() {
    let tree = TreeNode(1, TreeNode(2), TreeNode(3))
    assert(isSameTree(tree, tree))
    assert(!isSameTree(TreeNode(1, TreeNode(2), nil), TreeNode(1, nil, TreeNode(2))))
}
