/// https://leetcode.com/problems/delete-node-in-a-linked-list/
func deleteNode(_ node: ListNode?) {
    node?.val = (node?.next?.val)!
    node?.next = node?.next?.next
}

func testDeleteNodeInALinkedList() {
    let four = ListNode(4)
    let five = ListNode(5)
    let one = ListNode(1)
    let nine = ListNode(9)
    four.next = five
    five.next = one
    one.next = nine
    deleteNode(five)
    assert(four.val == 4)
    assert(four.next?.val == 1)
    assert(four.next?.next?.val == 9)
    assert(four.next?.next?.next == nil)
}
