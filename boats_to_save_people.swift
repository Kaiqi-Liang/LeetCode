/// https://leetcode.com/problems/boats-to-save-people/
class Solution {
    func numRescueBoats(_ people: [Int], _ limit: Int) -> Int {
        let people = people.sorted()
		var i = 0
		var j = people.count - 1
		var numBoats = 0
		while i < j {
			if people[j] + people[i] <= limit {
				i += 1
			}
			numBoats += 1
			j -= 1
		}
		if i == j {
			numBoats += 1
		}
		return numBoats
    }
}

assert(Solution().numRescueBoats([1,1,7,7], 8) == 2)
assert(Solution().numRescueBoats([3,5,3,4], 5) == 4)
assert(Solution().numRescueBoats([3,2,2,1], 3) == 3)
assert(Solution().numRescueBoats([1,2], 3) == 1)
