//! <https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/>
pub fn min_partitions(n: String) -> i32 {
    n.chars().map(|ch| ch.to_digit(10).unwrap()).max().unwrap() as _
}

fn main() {
    assert_eq!(min_partitions(String::from("32")), 3);
    assert_eq!(min_partitions(String::from("82734")), 8);
    assert_eq!(min_partitions(String::from("27346209830709182346")), 9);
}
