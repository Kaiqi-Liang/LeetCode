/// https://leetcode.com/problems/k-th-smallest-prime-fraction/
func maximumHappinessSum(_ happiness: [Int], _ k: Int) -> Int {
    return happiness.sorted().reversed().prefix(k).enumerated().reduce(0, { partialResult, value in
        partialResult + max(value.element - value.offset, 0)
    });
}

func testMaximizeHappinessOfSelectedChildren() {
    assert(maximumHappinessSum([12, 1, 42], 3) == 53);
    assert(maximumHappinessSum([1, 2, 3], 2) == 4);
    assert(maximumHappinessSum([1, 1, 1], 2) == 1);
}
