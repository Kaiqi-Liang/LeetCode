/**
 * https://leetcode.com/problems/is-subsequence/
 */
function isSubsequence(s: string, t: string): boolean {
    let i = 0;
    for (let j = 0; i < s.length && j < t.length; ++j) {
        if (s[i] === t[j]) {
            ++i;
        }
    }
    return i === s.length;
}

console.assert(isSubsequence('abc', 'ahbgdc'));
console.assert(!isSubsequence('axc', 'ahbgdc'));
