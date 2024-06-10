//! <https://leetcode.com/problems/perfect-number/>
pub fn check_perfect_number(num: i32) -> bool {
    match (1..=(num as f64).sqrt() as i32)
        .filter(|&i| num % i == 0 && i != num)
        .reduce(|a, b| a + b + num / b)
    {
        Some(sum) => sum == num,
        None => false,
    }
}

fn main() {
    assert!(check_perfect_number(6));
    assert!(check_perfect_number(28));
    assert!(!check_perfect_number(7));
    assert!(!check_perfect_number(1));
    assert!(!check_perfect_number(0));
    assert!(!check_perfect_number(-1));
}
