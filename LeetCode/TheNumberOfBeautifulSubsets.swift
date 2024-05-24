/// https://leetcode.com/problems/the-number-of-beautiful-subsets/
func beautifulSubsets(_ nums: [Int], _ k: Int) -> Int {
    var subsets = [[Int]]()
    for num in nums {
        for subset in subsets {
            let newSubset = subset + [num]
            if isBeautiful(newSubset) {
                subsets.append(newSubset)
            }
        }
        subsets.append([num])
    }
    func isBeautiful(_ nums: [Int]) -> Bool {
        let hashset = Set(nums)
        for num in nums {
            if hashset.contains(k + num) {
                return false
            }
        }
        return true
    }
    return subsets.count
}

func testTheNumberOfBeautifulSubsets() {
    assert(beautifulSubsets([2, 4, 6], 2) == 4)
    assert(beautifulSubsets([1], 1) == 1)
}
