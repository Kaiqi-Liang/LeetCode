/// https://leetcode.com/problems/distribute-coins-in-binary-tree/
func distributeCoins(_ root: TreeNode?) -> Int {
    var moves = 0
    func dfs(_ root: TreeNode?) -> Int {
        guard let node = root else { return 0 }
        let leftExtraCoins = dfs(node.left)
        let rightExtraCoins = dfs(node.right)
        moves += abs(leftExtraCoins) + abs(rightExtraCoins)
        return node.val + leftExtraCoins + rightExtraCoins - 1
    }
    guard dfs(root) == 0 else {
        fatalError()
    }
    return moves
}

func testDistributeCoinsInBinaryTree() {
    assert(distributeCoins(nil) == 0)
    assert(distributeCoins(TreeNode(3, TreeNode(), TreeNode())) == 2)
    assert(distributeCoins(TreeNode(0, TreeNode(3), TreeNode())) == 3)
    assert(distributeCoins(TreeNode(0, TreeNode(0, TreeNode(4), TreeNode()), nil)) == 5)
    assert(distributeCoins(TreeNode(0, TreeNode(1, TreeNode(3), TreeNode()), nil)) == 4)
}
