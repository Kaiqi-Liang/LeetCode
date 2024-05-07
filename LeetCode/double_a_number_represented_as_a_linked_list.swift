/// https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/
fileprivate func doubleIt(_ head: ListNode?) -> ListNode? {
    func recursive(_ head: ListNode, _ carry: inout Bool) -> ListNode? {
        if head.next != nil {
            head.next = recursive(head.next!, &carry)!
            head.next?.val *= 2
            if carry {
                head.next?.val += 1
            }
            if head.next!.val >= 10 {
                head.next?.val -= 10
                carry = true
            } else {
                carry = false
            }
        }
        return head
    }
    var carry = false
    var head = head
    head = recursive(head!, &carry)
    head?.val *= 2
    if carry {
        head?.val += 1
    }
    if head!.val >= 10 {
        head?.val -= 10
        let newNode = ListNode(1)
        newNode.next = head
        return newNode
    }
    return head
}

func testDoubleANumberRepresentedAsALinkedList() {
    let one = ListNode(1)
    let eight = ListNode(8)
    let nine = ListNode(9)
    one.next = eight
    eight.next = nine
    let res = doubleIt(one)
    assert(res?.val == 3);
    assert(res?.next?.val == 7)
    assert(res?.next?.next?.val == 8)
    assert(res?.next?.next?.next == nil)
}
