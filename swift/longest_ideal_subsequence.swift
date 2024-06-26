/// https://leetcode.com/problems/longest-ideal-subsequence/
class Solution {
    func longestIdealString(_ s: String, _ k: Int) -> Int {
        let numLowercaseLetters = 26
        var opt = Array(repeating: 0, count: numLowercaseLetters)
        for ch in s {
            var maxPrev = 0
            let index = Int(ch.asciiValue! - Character("a").asciiValue!)
            for i in -k...k {
                let prev = index + i
                if prev >= 0 && prev < numLowercaseLetters && opt[prev] > 0 {
                    maxPrev = max(maxPrev, opt[prev])
                }
            }
            opt[index] = maxPrev + 1
        }
        return opt.max()!
    }
}

assert(Solution().longestIdealString("acfgbd", 2) == 4)
assert(Solution().longestIdealString("abcd", 3) == 4)
