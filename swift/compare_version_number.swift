/// https://leetcode.com/problems/compare-version-numbers/description/
class Solution {
    func compareVersion(_ version1: String, _ version2: String) -> Int {
        let versions1 = version1.split(separator: ".").map{version in Int(version)!}
        let versions2 = version2.split(separator: ".").map{version in Int(version)!}
        for i in 0..<max(versions1.count, versions2.count) {
            let v1 = i >= versions1.count ? 0 : versions1[i]
            let v2 = i >= versions2.count ? 0 : versions2[i]
            if v1 < v2 {
                return -1
            } else if v1 > v2 {
                return 1
            }
        }
        return 0
    }
}

assert(Solution().compareVersion("1.01", "1.001") == 0)
assert(Solution().compareVersion("1.0", "1.0.0") == 0)
assert(Solution().compareVersion("0.1", "1.1") == -1)
