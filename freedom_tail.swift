/// https://leetcode.com/problems/freedom-trail/
class Solution {
    func findRotateSteps(_ ring: String, _ key: String) -> Int {
        let ring = Array(ring)
        let key = Array(key)
        var opt = Array(repeating: 0, count: ring.count)
        for i in (0..<key.count).reversed() {
            var curr_opt = Array(repeating: Int.max, count: ring.count)
            for j in 0..<ring.count {
                for k in 0..<ring.count {
                    if (ring[k] == key[i]) {
                        let dist = abs(j - k)
                        curr_opt[j] = min(curr_opt[j], min(dist, ring.count - dist) + opt[k])
                    }
                }
            }
            opt = curr_opt
        }
        return opt[0] + key.count
    }
}

assert(Solution().findRotateSteps("caotmcaataijjxi", "oatjiioicitatajtijciocjcaaxaaatmctxamacaamjjx") == 137)
assert(Solution().findRotateSteps("godding", "gd") == 4)
assert(Solution().findRotateSteps("godding", "godding") == 13)
