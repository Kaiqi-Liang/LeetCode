/// https://leetcode.com/problems/word-search/
class Solution {
    private func dfs(board: [[Character]], curr: Coordinate, index: Int, visited: inout Set<Coordinate>, word: String) -> Bool {
        if visited.contains(curr) || index >= word.count || word[String.Index(utf16Offset: index, in: word)] != board[curr.x][curr.y] {
            return false
        } else if index == word.count - 1 && word[String.Index(utf16Offset: index, in: word)] == board[curr.x][curr.y] {
            return true
        }
        visited.insert(curr)
        let neighbours = [Coordinate(x: 1, y: 0), Coordinate(x: 0, y: 1), Coordinate(x: -1, y: 0), Coordinate(x: 0, y: -1)]
        for neighbour in neighbours {
            let newCell = Coordinate(x: curr.x + neighbour.x, y: curr.y + neighbour.y)
            if newCell.x >= 0 && newCell.y >= 0 && newCell.x < board.count && newCell.y < board.first!.count && !visited.contains(newCell) {
                if dfs(board: board, curr: newCell, index: index + 1, visited: &visited, word: word) {
                    return true
                } else {
                    visited.remove(newCell)
                }
            }
        }
        return false
    }
    func exist(_ board: [[Character]], _ word: String) -> Bool {
        for char in word {
            var boardContainsWord = false
            for row in board {
                if row.contains(char) {
                    boardContainsWord = true
                    break
                }
            }
            if !boardContainsWord {
                return false
            }
        }
        for (i, row) in board.enumerated() {
            for (j, _) in row.enumerated() {
                var visited: Set<Coordinate> = []
                if dfs(board: board, curr: Coordinate(x: i, y: j), index: 0, visited: &visited, word: word) {
                    return true
                }
            }
        }
        return false
    }
}

struct Coordinate: Hashable {
    var x: Int
    var y: Int
}

assert(Solution().exist([["A","B","C","E"],
                         ["S","F","C","S"],
                         ["A","D","E","E"]], "ABCCED"))
assert(Solution().exist([["A","B","C","E"],
                         ["S","F","C","S"],
                         ["A","D","E","E"]], "SEE"))
assert(!Solution().exist([["A","B","C","E"],
                          ["S","F","C","S"],
                          ["A","D","E","E"]], "ABCB"))
assert(Solution().exist([["A","B","C","E"],
                         ["S","F","E","S"],
                         ["A","D","E","E"]], "ABCESEEEFS"))
assert(!Solution().exist([["A","A","A","A","A","A"],
                          ["A","A","A","A","A","A"],
                          ["A","A","A","A","A","A"],
                          ["A","A","A","A","A","A"],
                          ["A","A","A","A","A","A"],
                          ["A","A","A","A","A","A"]], "AAAAAAAAAAAAAAa"))
