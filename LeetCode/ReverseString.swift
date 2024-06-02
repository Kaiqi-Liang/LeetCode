func reverseString(_ s: inout [Character]) {
    for i in 0..<s.count / 2 {
        s.swapAt(i, s.count - i - 1)
    }
}

func testReverseString() {
    var input = [Character("h"), Character("e"), Character("l"), Character("l"), Character("o")]
    reverseString(&input)
    assert(input == [Character("o"), Character("l"), Character("l"), Character("e"), Character("h")])
}
