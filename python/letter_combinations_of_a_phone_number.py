"""
https://leetcode.com/problems/letter-combinations-of-a-phone-number/
"""


def letterCombinations(digits: str) -> list[str]:
    mapping = {
        2: "abc",
        3: "def",
        4: "ghi",
        5: "jkl",
        6: "mno",
        7: "pqrs",
        8: "tuv",
        9: "wxyz",
    }
    if not digits:
        return []

    combinations: list[str] = []

    def dfs(index: int, path: list[str]):
        if index == len(digits):
            combinations.append("".join(path))
        else:
            for letter in mapping[int(digits[index])]:
                path.append(letter)
                dfs(index + 1, path)
                path.pop()

    dfs(0, [])
    return combinations


if __name__ == "__main__":
    assert letterCombinations(
        "23") == ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    assert letterCombinations("") == []
    assert letterCombinations("2") == ["a", "b", "c"]
