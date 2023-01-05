//! <https://leetcode.com/problems/remove-nth-node-from-end-of-list/>
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut curr = head.as_mut();
    let mut sz = 0; // the number of nodes in the list
    while curr.is_some() {
        curr = curr.expect("curr.is_some()").next.as_mut();
        sz += 1;
    }

    let nth = sz - n;
    if nth == 0 {
        // first node is the one to be removed
        return head.expect("sz >= 1").next;
    }

    // find the node just before the one to be removed
    curr = head.as_mut();
    for _ in 0..nth - 1 {
        curr = curr.expect("n >= 1").next.as_mut();
    }

    let curr = curr.expect("curr is the node just before the one to be removed");
    let next = &curr
        .next
        .as_ref()
        .expect("curr.next is the node to be removed")
        .next;

    // curr.next = curr.next.next
    curr.next = next.as_ref().and(next.clone());
    head
}

fn main() {
    // remove_nth_from_end([1], 1) == []
    assert_eq!(
        remove_nth_from_end(Some(Box::new(ListNode { val: 1, next: None })), 1),
        None,
    );

    // remove_nth_from_end([1, 2], 1) == [1]
    assert_eq!(
        remove_nth_from_end(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            })),
            1,
        ),
        Some(Box::new(ListNode { val: 1, next: None })),
    );

    // remove_nth_from_end([1, 2], 2) == [2]
    assert_eq!(
        remove_nth_from_end(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            })),
            2,
        ),
        Some(Box::new(ListNode { val: 2, next: None })),
    );

    // remove_nth_from_end([1, 2, 3, 4, 5], 2) == [1, 2, 3, 5]
    assert_eq!(
        remove_nth_from_end(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            })),
            2,
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    );
}
