//! <https://leetcode.com/problems/merge-two-sorted-lists/>
use crate::list::ListNode;
use std::cmp;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list3: Option<Box<ListNode>> = None;
    let mut curr3 = list3.as_mut();
    let mut curr1 = list1;
    let mut curr2 = list2;
    while curr1.is_some() && curr2.is_some() {
        let ListNode {
            val: val1,
            next: next1,
        } = *curr1.clone().unwrap();

        let ListNode {
            val: val2,
            next: next2,
        } = *curr2.clone().unwrap();

        let item = Some(Box::new(ListNode {
            val: cmp::min(val1, val2),
            next: None,
        }));

        if val1 < val2 {
            curr1 = next1
        } else {
            curr2 = next2
        }

        match curr3 {
            Some(curr) => {
                curr.next = item;
                curr3 = curr.next.as_mut();
            }
            None => {
                list3 = item;
                curr3 = list3.as_mut();
            }
        }
    }
    let curr = if curr1.is_some() { curr1 } else { curr2 };
    match curr3 {
        Some(list) => (*list).next = curr,
        None => list3 = curr,
    }
    list3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lists() {
        assert_eq!(merge_two_lists(None, None), None);
    }

    #[test]
    fn one_list_empty() {
        assert_eq!(
            merge_two_lists(None, Some(Box::new(ListNode::new(0)))),
            Some(Box::new(ListNode::new(0)))
        );
    }

    #[test]
    fn happy_path() {
        assert_eq!(
            merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 4, next: None }))
                            }))
                        }))
                    }))
                }))
            }))
        );
    }
}
