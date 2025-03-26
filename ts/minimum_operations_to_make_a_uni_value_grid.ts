/**
 * https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/
 * @date 2025-03-26
 */

function minOperations(grid: number[][], x: number): number {
    const numbers = grid.flat();
    if (new Set(numbers.map((num) => num % x)).size > 1) return -1;
    numbers.sort((a, b) => a - b);
    const medium = numbers[Math.floor(numbers.length / 2)]
    return numbers.reduce((a, c) => a + Math.abs(c - medium) / x, 0);
};

console.assert(minOperations([[2, 4], [6, 8]], 2) == 4);
console.assert(minOperations([[1, 5], [2, 3]], 1) == 5);
console.assert(minOperations([[1, 2], [3, 4]], 2) == -1);
console.assert(minOperations([[980, 476, 644, 56], [644, 140, 812, 308], [812, 812, 896, 560], [728, 476, 56, 812]], 84) == 45);
