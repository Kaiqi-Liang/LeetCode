/// https://leetcode.com/problems/sum-of-all-subset-xor-totals/
func subsetXORSum(_ nums: [Int]) -> Int {
    var subsets = [[Int]]()
    func xor(_ nums: [Int], _ subsets: inout [[Int]], _ curr: [Int]) {
        if nums.isEmpty {
            if !curr.isEmpty {
                subsets.append(curr)
            }
            return
        }
        xor(Array(nums.suffix(from: 1)), &subsets, curr)
        xor(Array(nums.suffix(from: 1)), &subsets, curr + [nums.first!])
    }
    xor(nums, &subsets, [])
    return subsets.map { subset in
        subset.reduce(0, ^)
    }.reduce(0, +)
}

func testSumOfAllSubsetXORTotals() {
    assert(subsetXORSum([1, 3]) == 6)
    assert(subsetXORSum([5, 1, 6]) == 28)
    assert(subsetXORSum([3, 4, 5, 6, 7, 8]) == 480)
}
