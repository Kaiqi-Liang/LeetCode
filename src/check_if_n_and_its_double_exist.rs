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
mod set {
    use std::collections::HashSet;
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for i in arr {
            if set.contains(&(i * 2)) || i % 2 == 0 && set.contains(&(i / 2)) {
                return true;
            }
            set.insert(i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        assert!(sort::check_if_exist(vec![10, 2, 5, 3]));
        assert!(!sort::check_if_exist(vec![3, 1, 7, 11]));
    }

    #[test]
    fn set() {
        assert!(set::check_if_exist(vec![10, 2, 5, 3]));
        assert!(!set::check_if_exist(vec![3, 1, 7, 11]));
    }
}
