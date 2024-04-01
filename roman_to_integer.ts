/**
 * https://leetcode.com/problems/roman-to-integer/
 * @date 2022-07-16
 */

function romanToInt(s: string): number {
    const lookup_table = {
        'I': 1,
        'V': 5,
        'X': 10,
        'L': 50,
        'C': 100,
        'D': 500,
        'M': 1000,
    };
    let res = 0;
    for (let index = 0; index < s.length; ++index) {
        let element = lookup_table[s[index]];
        if (index < s.length - 1) {
            const next = lookup_table[s[index + 1]];
            if (next > element) {
                element = next - element;
                ++index;
            }
        }
        res += element;
    }
    return res;
};

console.assert(romanToInt("II") == 2);
console.assert(romanToInt("VI") == 6);
console.assert(romanToInt("XXVII") == 27);
console.assert(romanToInt("IV") == 4);
