struct TreeNode {
	int val;
	TreeNode *left;
	TreeNode *right;
	TreeNode()
	: val(0)
	, left(nullptr)
	, right(nullptr) {}
	TreeNode(int x)
	: val(x)
	, left(nullptr)
	, right(nullptr) {}
	TreeNode(int x, TreeNode *left, TreeNode *right)
	: val(x)
	, left(left)
	, right(right) {}
};

bool isSameTree(TreeNode *p, TreeNode *q) {
	if (p == nullptr and q == nullptr)
		return true;
	else if (p == nullptr or q == nullptr)
		return false;
	if (p->val != q->val)
		return false;
	else
		return isSameTree(p->left, q->left) and isSameTree(p->right, q->right);
}
