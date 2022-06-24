/**
 * https://leetcode.com/problems/letter-combinations-of-a-phone-number/
 * @date 2022-06-24
 */
type Digit = 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;

interface Tree {
    char: string;
    children: Tree[];
}

function letterCombinations(digits: string): string[] {
    const numbers = digits.split('').map((d) => parseInt(d) as Digit);
    const result: string[] = [];

    if (numbers.length > 0) {
        const mapping = {
            2: ['a', 'b', 'c'],
            3: ['d', 'e', 'f'],
            4: ['g', 'h', 'i'],
            5: ['j', 'k', 'l'],
            6: ['m', 'n', 'o'],
            7: ['p', 'q', 'r', 's'],
            8: ['t', 'u', 'v'],
            9: ['w', 'x', 'y', 'z']
        };

        const generateTree = (tree: Tree, digits: Digit[]) => {
            if (digits.length > 0) {
                for (const letter of mapping[digits[0]]) {
                    const child = generateTree({
                        char: letter,
                        children: [],
                    }, digits.slice(1));
                    tree.children.push(child);
                }
            }
            return tree;
        };

        const generateResult = (tree: Tree, letters: string) => {
            if (tree.children.length === 0) {
                result.push(letters + tree.char);
            } else {
                for (const child of tree.children) {
                    generateResult(child, letters + tree.char);
                }
            }
        };

        generateResult(generateTree({
            char: '',
            children: [],
        }, numbers), '');
    }
    return result;
}

console.assert(JSON.stringify(letterCombinations('')) == JSON.stringify([]));
console.assert(JSON.stringify(letterCombinations('2')) == JSON.stringify(['a','b','c']));
console.assert(JSON.stringify(letterCombinations('23')) == JSON.stringify(['ad','ae','af','bd','be','bf','cd','ce','cf']));
console.assert(JSON.stringify(letterCombinations('222')) == JSON.stringify(['aaa','aab','aac','aba','abb','abc','aca','acb','acc','baa','bab','bac','bba','bbb','bbc','bca','bcb','bcc','caa','cab','cac','cba','cbb','cbc','cca','ccb','ccc']));
