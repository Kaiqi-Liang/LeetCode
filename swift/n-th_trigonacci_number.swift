/// https://leetcode.com/problems/n-th-tribonacci-number/
class Solution {
    private var cache = Array(repeating: -1, count: 38)
    init() {
        cache[0] = 0
        cache[1] = 1
        cache[2] = 1
    }
    func tribonacci(_ n: Int) -> Int {
        if cache[n] == -1 {
            cache[n] = tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3)
        }
        return cache[n] 
    }
}

assert(Solution().tribonacci(2) == 1)
assert(Solution().tribonacci(3) == 2)
assert(Solution().tribonacci(4) == 4)
assert(Solution().tribonacci(25) == 1389537)
