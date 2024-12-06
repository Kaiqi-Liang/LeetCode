//! <https://leetcode.com/problems/remove-duplicates-from-sorted-array/>
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    for j in 1..nums.len() {
        if nums[j] != nums[i] {
            i += 1;
            nums[i] = nums[j];
        }
    }
    i as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    fn judge(nums: &mut Vec<i32>, expected: Vec<i32>) {
        let result = remove_duplicates(nums) as usize;
        assert_eq!(result, expected.len());
        for i in 0..result {
            assert_eq!(nums[i], expected[i]);
        }
    }

    #[test]
    fn happy_path() {
        judge(&mut vec![1, 1, 2], vec![1, 2]);
    }

    #[test]
    fn multiple_duplicates() {
        judge(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]);
    }
}
