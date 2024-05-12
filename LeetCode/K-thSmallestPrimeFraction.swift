/// https://leetcode.com/problems/k-th-smallest-prime-fraction/
func kthSmallestPrimeFraction(_ arr: [Int], _ k: Int) -> [Int] {
    var fractions = [(Double, Int, Int)]()
    for i in 0..<arr.count {
        for j in (i + 1)..<arr.count {
            fractions.append((Double(arr[i]) / Double(arr[j]), arr[i], arr[j]))
        }
    }
    let res = fractions.sorted { a, b in
        a.0 < b.0
    }[k - 1]
    return [res.1, res.2]
}

func testKthSmallestPrimeFraction() {
    assert(kthSmallestPrimeFraction([1, 2, 3, 5], 3) == [2, 5])
    assert(kthSmallestPrimeFraction([1, 7], 1) == [1, 7])
}
