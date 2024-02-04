/// https://leetcode.com/problems/single-number-ii/
class Solution {
    func singleNumber(_ nums: [Int]) -> Int {
        var map: [Int: UInt] = [:]
		for num in nums {
			if let count = map[num] {
				map.updateValue(count + 1, forKey: num)
			} else {
				map.updateValue(1, forKey: num)
			}
		}
		return map.filter { (key, value) -> Bool in
		    return value != 3
		}.first!.key
	}
}

assert(Solution().singleNumber([2, 2, 3, 2]) == 3)
assert(Solution().singleNumber([0, 1, 0, 1, 0, 1, 99]) == 99)
