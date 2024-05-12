/// https://leetcode.com/problems/largest-local-values-in-a-matrix/
func largestLocal(_ grid: [[Int]]) -> [[Int]] {
    var maxLocal = Array(repeating: Array(repeating: 0, count: grid.count - 2), count: grid.count - 2)
    for row in 0..<maxLocal.count {
        for col in 0..<maxLocal.count {
            var largestValue = 0
            for i in 0..<3 {
                for j in 0..<3 {
                    largestValue = max(largestValue, grid[row + i][col + j])
                }
            }
            maxLocal[row][col] = largestValue
        }
    }
    return maxLocal
}

func testLargestLocalValuesInAMatrix() {
    assert(largestLocal([[9, 9, 8, 1],
                         [5, 6, 2, 6],
                         [8, 2, 6, 4],
                         [6, 2, 2, 2]]) == [[9, 9],
                                            [8, 6]])
    assert(largestLocal([[1, 1, 1, 1, 1],
                         [1, 1, 1, 1, 1],
                         [1, 1, 2, 1, 1],
                         [1, 1, 1, 1, 1],
                         [1, 1, 1, 1, 1]]) == [[2, 2, 2],
                                               [2, 2, 2],
                                               [2, 2, 2]])
}
