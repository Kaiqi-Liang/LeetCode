/// https://leetcode.com/problems/score-of-a-string/
func scoreOfString(_ s: String) -> Int {
    var score = 0
    for i in 1..<s.count {
        let a = Int(s[String.Index(utf16Offset: i, in: s)].asciiValue!)
        let b = Int(s[String.Index(utf16Offset: i - 1, in: s)].asciiValue!)
        score += abs(a - b)
    }
    return score
}

func testScoreOfAString() {
    assert(scoreOfString("hello") == 13)
    assert(scoreOfString("zaz") == 50)
}
