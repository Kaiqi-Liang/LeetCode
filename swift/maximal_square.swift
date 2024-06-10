/// https://leetcode.com/problems/maximal-square/
class Solution {
    func maximalSquare(_ matrix: [[Character]]) -> Int {
        var maxArea = 0
        let rows = matrix.count
        let cols = matrix.first!.count
        var opt = Array(repeating: Array(repeating: 0, count: cols), count: rows)
        for row in (0..<rows).reversed() {
            for col in (0..<cols).reversed() {
                if matrix[row][col] == "1" {
                    if row == rows - 1 || col == cols - 1 {
                        opt[row][col] = 1
                    } else {
                        opt[row][col] = min(opt[row + 1][col + 1], opt[row + 1][col], opt[row][col + 1]) + 1
                    }
                    maxArea = max(maxArea, opt[row][col] * opt[row][col])
                }
            }
        }
        return maxArea
    }
}

assert(Solution().maximalSquare([["1","1","1","1","0"],
                                 ["1","1","1","1","0"],
                                 ["1","1","1","1","1"],
                                 ["1","1","1","1","1"],
                                 ["0","0","1","1","1"]]) == 16)
assert(Solution().maximalSquare([["1","0","1","0","0"],
                                 ["1","0","1","1","1"],
                                 ["1","1","1","1","1"],
                                 ["1","0","0","1","0"]]) == 4)
assert(Solution().maximalSquare([["0","1"],["1","0"]]) == 1)
assert(Solution().maximalSquare([["0"]]) == 0)
