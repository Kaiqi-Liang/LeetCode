"""
https://leetcode.com/problems/valid-word-abbreviation/
"""
def check_abbr(word: str, abbr: str) -> bool:
    curr_count = ""
    i = 0
    for j in range(len(abbr)):
        if abbr[j].isdigit():
            if not curr_count and abbr[j] == "0":
                return False
            curr_count += abbr[j]
            continue
        elif curr_count:
            i += int(curr_count)
            if i >= len(word):
                return False
            curr_count = ""
        if word[i] != abbr[j]:
            return False
        i += 1
    return i == len(word)
