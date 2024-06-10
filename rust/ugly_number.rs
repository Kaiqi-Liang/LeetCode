//! <https://leetcode.com/problems/ugly-number/>
pub fn is_ugly(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut n = n;
    let mut i = 2;
    while i <= n {
        while n % i != 0 {
            if i > 5 {
                return false;
            }
            i += 1;
        }
        n /= i;
    }
    true
}

fn main() {
    assert!(is_ugly(6));
    assert!(is_ugly(1));
    assert!(!is_ugly(14));
    assert!(!is_ugly(-1));
    assert!(!is_ugly(0));
    assert!(!is_ugly(1332185066));
}
