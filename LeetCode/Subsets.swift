/// https://leetcode.com/problems/subsets/
func subsets(_ nums: [Int]) -> [[Int]] {
    var subsets: [[Int]] = [[]]
    for num in nums {
        for subset in subsets {
            subsets.append(subset + [num])
        }
    }
    return subsets
}

func testSubsets() {
    assert(subsets([1, 2, 3]) == [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]])
    assert(subsets([0]) == [[], [0]])
}
