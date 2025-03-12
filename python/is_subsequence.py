"""
https://leetcode.com/problems/is-subsequence/
"""


def is_subsequence(s: str, t: str) -> bool:
    if not s:
        return True
    i = 0
    for j in range(len(t)):
        if s[i] == t[j]:
            i += 1
            if i == len(s):
                return True
    return False


if __name__ == "__main__":
    assert is_subsequence("abc", "ahbgdc")
    assert not is_subsequence("axc", "ahbgdc")
    assert is_subsequence("", "ahbgdc")
    assert is_subsequence("", "")
    assert not is_subsequence("ahbgdc", "")
