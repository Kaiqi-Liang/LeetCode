//! <https://leetcode.com/problems/minimum-cost-to-hire-k-workers/>
use std::collections::BinaryHeap;
pub fn mincost_to_hire_workers(quality: Vec<u32>, wage: Vec<u32>, k: usize) -> f64 {
    let mut ratio: Vec<(f64, u32)> = quality
        .iter()
        .zip(wage)
        .map(|(&quality, wage)| (wage as f64 / quality as f64, quality))
        .collect();
    ratio.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut max_quantity = BinaryHeap::new();
    let mut res = f64::MAX;
    let mut sum_quality = 0;
    for (i, (ratio, quantity)) in ratio.into_iter().enumerate() {
        sum_quality += quantity;
        if i >= k - 1 {
            if i >= k {
                sum_quality -= max_quantity.pop().expect("k > 0");
            }
            res = res.min(ratio * sum_quality as f64);
        }
        max_quantity.push(quantity);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert!(
            (mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2) - 105.00000).abs()
                < 10e-5
        );
    }

    #[test]
    fn not_divisible() {
        assert!(
            (mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3) - 30.66667)
                .abs()
                < 10e-5
        );
    }
}
