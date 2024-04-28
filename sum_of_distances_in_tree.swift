/// https://leetcode.com/problems/sum-of-distances-in-tree/
class Solution {
    func sumOfDistancesInTree(_ n: Int, _ edges: [[Int]]) -> [Int] {
        var graph = Array(repeating: Array<Int>(), count: n)
        for edge in edges {
            graph[edge[0]].append(edge[1])
            graph[edge[1]].append(edge[0])
        }

        var numNodes = Array(repeating: 1, count: n)
        var sumDistances = Array(repeating: 0, count: n)
        var visited = Array(repeating: false, count: n)
        func getNumNodesPerSubtree(_ node: Int) -> Int {
            visited[node] = true
            var numChildren = 0
            for neighbour in graph[node] {
                if !visited[neighbour] {
                    numChildren += getNumNodesPerSubtree(neighbour)
                }
            }
            numNodes[node] += numChildren
            sumDistances[0] += numChildren
            return numNodes[node]
        }
        numNodes[0] = getNumNodesPerSubtree(0)

        visited = Array(repeating: false, count: n)
        func getSumDistancesPerNode(_ node: Int, _ currSumDistance: Int) {
            visited[node] = true
            sumDistances[node] = currSumDistance
            for neighbour in graph[node] {
                if !visited[neighbour] {
                    getSumDistancesPerNode(neighbour, sumDistances[node] + n - numNodes[neighbour] - numNodes[neighbour])
                }
            }
        }
        getSumDistancesPerNode(0, sumDistances[0])
        return sumDistances
    }
}

assert(Solution().sumOfDistancesInTree(6, [[0,1],
                                           [0,2],
                                           [2,3],
                                           [2,4],
                                           [2,5]]) == [8,12,6,10,10,10])
assert(Solution().sumOfDistancesInTree(1, []) == [0])
assert(Solution().sumOfDistancesInTree(2, [1, 0]) == [1, 1])
