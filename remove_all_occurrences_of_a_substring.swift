/// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
class Solution {
    func removeOccurrences(_ s: String, _ part: String) -> String {
        var s = s
        while let i = s.firstRange(of: part) {
            s.removeSubrange(i)
        }
        return s
    }
}

assert(Solution().removeOccurrences("daabcbaabcbc", "abc") == "dab")
assert(Solution().removeOccurrences("axxxxyyyyb", "xy") == "ab")
