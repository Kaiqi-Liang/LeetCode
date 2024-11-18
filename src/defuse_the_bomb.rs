/// <https://leetcode.com/problems/defuse-the-bomb/>
pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    (0..code.len())
        .map(|index| match k.cmp(&0) {
            std::cmp::Ordering::Less => {
                let k = (-k) as usize;
                code[if k > index { 0 } else { 0.max(index - k) }..index]
                    .iter()
                    .sum::<i32>()
                    + if k > index {
                        code.iter().skip(code.len() - (k - index)).sum::<i32>()
                    } else {
                        0
                    }
            }
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => {
                let k = k as usize;
                code[index + 1..=(code.len() - 1).min(index + k)]
                    .iter()
                    .sum::<i32>()
                    + if index + k < code.len() {
                        0
                    } else {
                        code[0..=(index + k) % code.len()].iter().sum::<i32>()
                    }
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn positive_k() {
        assert_eq!(decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
    }

    #[test]
    fn zero_k() {
        assert_eq!(decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
    }

    #[test]
    fn negative_k() {
        assert_eq!(decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }

    #[test]
    fn happy_path() {
        assert_eq!(decrypt(vec![5, 2, 2, 3, 1], 3), vec![7, 6, 9, 8, 9]);
    }
}
