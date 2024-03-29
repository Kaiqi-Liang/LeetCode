//! <https://leetcode.com/problems/climbing-stairs/>
/// `O(n)` space complexity
mod array {
    pub fn climb_stairs(n: i32) -> i32 {
        let n: usize = n as _;
        let mut opt = vec![1; n + 1];
        for i in (0..n - 1).rev() {
            opt[i] = opt[i + 1] + opt[i + 2];
        }
        *opt.first().expect("opt.len() >= 1")
    }
}

/// `O(1)` space complexity
mod variables {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut one, mut two) = (1, 1);
        for _ in 0..n - 1 {
            let sum = one + two;
            two = one;
            one = sum;
        }
        one
    }
}

fn main() {
    assert_eq!(array::climb_stairs(1), variables::climb_stairs(1));
    assert_eq!(array::climb_stairs(2), variables::climb_stairs(2));
    assert_eq!(array::climb_stairs(3), variables::climb_stairs(3));
    assert_eq!(array::climb_stairs(4), variables::climb_stairs(4));
    assert_eq!(array::climb_stairs(5), variables::climb_stairs(5));
}
