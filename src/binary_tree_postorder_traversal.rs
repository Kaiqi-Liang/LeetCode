//! <https://leetcode.com/problems/binary-tree-postorder-traversal/>
use crate::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if let Some(node) = root {
        let mut res = postorder_traversal(node.borrow().left.clone());
        res.append(&mut postorder_traversal(node.borrow().right.clone()));
        res.push(node.borrow().val);
        res
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            postorder_traversal(Some(Rc::from(RefCell::from(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::from(RefCell::from(TreeNode {
                    val: 2,
                    left: Some(Rc::from(RefCell::from(TreeNode::new(3)))),
                    right: None,
                }))),
            })))),
            vec![3, 2, 1],
        );
    }

    #[test]
    fn empty_tree() {
        assert_eq!(postorder_traversal(None), Vec::new());
    }

    #[test]
    fn only_root_node() {
        assert_eq!(
            postorder_traversal(Some(Rc::from(RefCell::from(TreeNode {
                val: 1,
                left: None,
                right: None
            })))),
            vec![1],
        );
    }
}
