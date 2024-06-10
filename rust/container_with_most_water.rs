//! <https://leetcode.com/problems/container-with-most-water/>
use std::cmp;
pub fn max_area(height: Vec<usize>) -> usize {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut area = 0;
    while l < r {
        area = cmp::max(area, (r - l) * cmp::min(height[l], height[r]));
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    area
}

fn main() {
    assert_eq!(max_area(vec![1, 1]), 1);
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(max_area(vec![1, 5, 4, 1]), 4);
}
