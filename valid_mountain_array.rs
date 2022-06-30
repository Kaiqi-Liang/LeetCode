//! <https://leetcode.com/problems/valid-mountain-array/>
pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    if arr.len() < 3 {
        return false;
    }
    let mut i = 0;
    let i = loop {
        if i == arr.len() - 1 {
            return false;
        } else if arr[i] >= arr[i + 1] {
            break i;
        }
        i += 1;
    };
    if i == 0 {
        return false;
    }
    for i in i..arr.len() - 1 {
        if arr[i] <= arr[i + 1] {
            return false;
        }
    }
    true
}

fn main() {
    assert!(valid_mountain_array(vec![0, 1, 2, 1, 0]));
    assert!(valid_mountain_array(vec![0, 1, 2, 1]));
    assert!(!valid_mountain_array(vec![0, 1, 2, 3]));
    assert!(!valid_mountain_array(vec![3, 2, 1]));
    assert!(!valid_mountain_array(vec![3, 3, 2, 1]));
    assert!(valid_mountain_array(vec![0, 3, 2, 1]));
    assert!(!valid_mountain_array(vec![1, 2]));
    assert!(!valid_mountain_array(vec![3, 5, 5]));
}
