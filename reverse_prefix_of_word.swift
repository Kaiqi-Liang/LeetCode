/// https://leetcode.com/problems/reverse-prefix-of-word/description/
class Solution {
    func reversePrefix(_ word: String, _ ch: Character) -> String {
        if let first = word.firstIndex(of: ch) {
            return String(word.prefix(through: first).reversed() + word.suffix(from: word.index(first, offsetBy: 1)))
        } else {
            return word
        }
    }
}

assert(Solution().reversePrefix("abcdefd", "d") == "dcbaefd")
assert(Solution().reversePrefix("xyxzxe", "z") == "zxyxxe")
assert(Solution().reversePrefix("abcd", "z") == "abcd")
