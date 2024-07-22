//! <https://leetcode.com/problems/remove-nodes-from-linked-list/>
use crate::list::ListNode;
use std::collections::VecDeque;

fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut monotonic_stack = VecDeque::new();
    let mut curr = head.as_ref();
    while let Some(node) = curr {
        let val = node.val;
        while let Some(&top) = monotonic_stack.back() {
            if val > top {
                monotonic_stack.pop_back();
            } else {
                break;
            }
        }
        monotonic_stack.push_back(val);
        curr = node.next.as_ref();
    }
    let mut head = Some(Box::new(ListNode::new(
        monotonic_stack
            .pop_front()
            .expect("There must be at least one value because push_back is called after pop_back on every iteration"),
    )));
    let mut curr = head.as_mut();
    while let Some(top) = monotonic_stack.pop_front() {
        curr.as_mut()
            .expect("It is created in the last iteration or before the loop started")
            .next = Some(Box::new(ListNode::new(top)));
        curr = curr
            .expect("It is created in the last iteration or before the loop started")
            .next
            .as_mut();
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            remove_nodes(Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 13,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 8, next: None })),
                        })),
                    })),
                })),
            }))),
            Some(Box::new(ListNode {
                val: 13,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            }))
        );
    }

    #[test]
    fn nothing_to_remove() {
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert_eq!(remove_nodes(expected.clone()), expected);
    }
}
