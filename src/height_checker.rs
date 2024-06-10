//! <https://leetcode.com/problems/height-checker/>
pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut expected = heights.clone();
    expected.sort();
    expected
        .into_iter()
        .zip(heights)
        .filter(|(expected, height)| expected != height)
        .count() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }

    #[test]
    fn no_match() {
        assert_eq!(height_checker(vec![5, 1, 2, 3, 4]), 5);
    }

    #[test]
    fn all_match() {
        assert_eq!(height_checker(vec![1, 2, 3, 4, 5]), 0);
    }
}
