//! https://leetcode.com/problems/merge-sorted-array/
use std::cmp;
pub fn merge(nums1: &mut Vec<usize>, m: usize, nums2: &mut Vec<usize>, n: usize) {
    let mut copy = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < m && j < n {
        copy.push(cmp::min(nums1[i], nums2[j]));
        if nums1[i] < nums2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    copy.append(&mut nums1[i..m].to_vec());
    copy.append(&mut nums2[j..n].to_vec());
    nums1.clone_from(&copy);
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

    let mut nums1 = vec![2, 0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 1, &mut nums2, 1);
    assert_eq!(nums1, vec![1, 2]);
}
