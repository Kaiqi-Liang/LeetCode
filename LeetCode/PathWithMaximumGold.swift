/// https://leetcode.com/problems/path-with-maximum-gold/
func getMaximumGold(_ grid: [[Int]]) -> Int {
    var max_gold = 0
    for (i, row) in grid.enumerated() {
        for col in 0..<row.count {
            var visited: Set<Coordinate> = []
            dfs(Coordinate(x: i, y: col), &visited, grid[i][col], &max_gold)
        }
    }
    func dfs(_ curr: Coordinate, _ visited: inout Set<Coordinate>, _ gold: Int, _ maxGold: inout Int) {
        if visited.contains(curr) || grid[curr.x][curr.y] == 0 {
            return
        }
        visited.insert(curr)
        for neighbour in NEIGHBOURS {
            let newCell = Coordinate(x: curr.x + neighbour.x, y: curr.y + neighbour.y)
            if newCell.x >= 0 && newCell.y >= 0 && newCell.x < grid.count && newCell.y < grid.first!.count && !visited.contains(newCell) && grid[newCell.x][newCell.y] > 0 {
                dfs(newCell, &visited, gold + grid[newCell.x][newCell.y], &maxGold)
            }
        }
        maxGold = max(maxGold, gold)
        visited.remove(curr)
    }
    return max_gold
}

func testPathWithMaximumGold() {
    assert(getMaximumGold([[0,6,0],
                           [5,8,7],
                           [0,9,0]]) == 24)
    assert(getMaximumGold([[1,0,7],
                           [2,0,6],
                           [3,4,5],
                           [0,3,0],
                           [9,0,20]]) == 28)
}
