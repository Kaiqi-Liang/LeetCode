/// https://leetcode.com/problems/delete-node-in-a-linked-list/
class Solution {
    func deleteNode(_ node: ListNode?) {
        node?.val = (node?.next?.val)!
        node?.next = node?.next?.next
    }
}

public class ListNode {
    public var val: Int
    public var next: ListNode?
    public init(_ val: Int) {
        self.val = val
        self.next = nil
    }
}

let four = ListNode(4)
let five = ListNode(5)
let one = ListNode(1)
let nine = ListNode(9)
four.next = five
five.next = one
one.next = nine
Solution().deleteNode(five)
assert(four.val == 4)
assert(four.next?.val == 1)
assert(four.next?.next?.val == 9)
assert(four.next?.next?.next == nil)
