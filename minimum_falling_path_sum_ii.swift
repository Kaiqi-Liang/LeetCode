/// https://leetcode.com/problems/minimum-falling-path-sum-ii/
class Solution {
    func minFallingPathSum(_ grid: [[Int]]) -> Int {
        var opt = grid[0]
        for row in 1..<grid.count {
            var curr_opt = Array(repeating: Int.max, count: grid.count)
            let min_column = opt.min()!
            let min_index = opt.firstIndex(of: min_column)!
            let second_min_column = opt.enumerated().filter { $0.offset != min_index }.map { $0.element }.min()!
            for col in 0..<grid.count {
                curr_opt[col] = grid[row][col] + (min_index == col ? second_min_column : min_column)
            }
            opt = curr_opt
        }
        return opt.min()!
    }
}

assert(Solution().minFallingPathSum([[1,2,3],
                                     [4,5,6],
                                     [7,8,9]]) == 13)
assert(Solution().minFallingPathSum([[7]]) == 7)
