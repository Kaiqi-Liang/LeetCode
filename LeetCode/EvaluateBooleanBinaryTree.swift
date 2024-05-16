func evaluateTree(_ root: TreeNode?) -> Bool {
    if let node = root {
        switch node.val {
        case 0: false
        case 1: true
        case 2: evaluateTree(node.left) || evaluateTree(node.right)
        case 3: evaluateTree(node.left) && evaluateTree(node.right)
        default: false
        }
    } else {
        false
    }
}

func testEvaluateBooleanBinaryTree() {
    assert(!evaluateTree(TreeNode(0)))
    assert(evaluateTree(TreeNode(1)))
    assert(evaluateTree(TreeNode(2, TreeNode(1), TreeNode(3, TreeNode(0), TreeNode(1)))))
}
