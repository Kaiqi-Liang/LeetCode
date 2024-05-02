/// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/
class Solution {
    func findMaxK(_ nums: [Int]) -> Int {
        var positives = Set<Int>()
        var negatives = Set<Int>()
        for num in nums {
            if num > 0 {
                positives.insert(num)
            } else if num < 0 {
                negatives.insert(num)
            }
        }
        var max_positive = -1;
        for positive in positives {
            if negatives.contains(positive * -1) {
                max_positive = max(positive, max_positive)
            }
        }
        return max_positive
    }
}

assert(Solution().findMaxK([-1,2,-3,3]) == 3)
assert(Solution().findMaxK([-1,10,6,7,-7,1]) == 7)
assert(Solution().findMaxK([-10,8,6,7,-2,-3]) == -1)
