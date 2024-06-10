//! <https://leetcode.com/problems/check-if-n-and-its-double-exist/>
/// `O(nlogn)` time complexity
mod sort {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();
        for &elem in arr.iter() {
            if let Ok(idx) = arr.binary_search(&(2 * elem)) {
                if elem != arr[idx] || arr.iter().filter(|&&n| n == elem).count() > 1 {
                    return true;
                }
            }
        }
        false
    }
}

/// `O(n)` expected time complexity
mod map {
    use std::collections::HashSet;
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut map = HashSet::new();
        for i in arr {
            if map.contains(&(i * 2)) || i % 2 == 0 && map.contains(&(i / 2)) {
                return true;
            }
            map.insert(i);
        }
        false
    }
}

fn main() {
    assert_eq!(
        sort::check_if_exist(vec![3, 1, 7, 11]),
        map::check_if_exist(vec![3, 1, 7, 11])
    );
    assert_eq!(
        sort::check_if_exist(vec![10, 2, 5, 3]),
        map::check_if_exist(vec![10, 2, 5, 3])
    );
    assert_eq!(
        sort::check_if_exist(vec![7, 1, 14, 11]),
        map::check_if_exist(vec![7, 1, 14, 11])
    );
    assert_eq!(
        sort::check_if_exist(vec![0, 0]),
        map::check_if_exist(vec![0, 0])
    );
    assert_eq!(
        sort::check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]),
        map::check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8])
    );
}
