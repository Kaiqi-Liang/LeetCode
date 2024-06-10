/// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x
func specialArray(_ nums: [Int]) -> Int {
    for x in 0...nums.count {
        var numGreaterThanOrEqualToX = 0
        for num in nums {
            if num >= x {
                numGreaterThanOrEqualToX += 1
            }
        }
        if numGreaterThanOrEqualToX == x {
            return x
        }
    }
    return -1
}

func testSpecialArrayWithXElementsGreaterThanOrEqualX() {
    assert(specialArray([3, 5]) == 2)
    assert(specialArray([0, 0]) == -1)
    assert(specialArray([0, 4, 3, 0, 4]) == 3)
}
