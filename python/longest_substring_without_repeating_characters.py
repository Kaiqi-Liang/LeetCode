"""
https://leetcode.com/problems/longest-substring-without-repeating-characters/
"""


def lengthOfLongestSubstring(s: str) -> int:
    left = 0
    count: set[str] = set()
    max_length = 0
    for right in range(len(s)):
        if s[right] in count:
            while True:
                count.remove(s[left])
                left += 1
                if s[right] == s[left - 1]:
                    break
        count.add(s[right])
        max_length = max(max_length, right - left + 1)
    return max_length


if __name__ == '__main__':
    assert lengthOfLongestSubstring("tmmzuxt") == 5
    assert lengthOfLongestSubstring("abcabcbb") == 3
