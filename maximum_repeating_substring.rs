//! <https://leetcode.com/problems/maximum-repeating-substring/>
pub fn max_repeating(sequence: String, word: String) -> i32 {
    let mut count = 1;
    while sequence.contains(&word.repeat(count)) {
        count += 1;
    }
    (count - 1) as i32
}

fn main() {
    assert_eq!(max_repeating(String::from("ababc"), String::from("ab")), 2);
    assert_eq!(max_repeating(String::from("ababc"), String::from("ba")), 1);
    assert_eq!(max_repeating(String::from("ababc"), String::from("ac")), 0);
    assert_eq!(
        max_repeating(
            String::from("aaabaaaabaaabaaaabaaaabaaaabaaaaba"),
            String::from("aaaba")
        ),
        5
    );
}
