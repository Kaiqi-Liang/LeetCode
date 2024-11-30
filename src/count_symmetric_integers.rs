//! <https://leetcode.com/problems/count-symmetric-integers/>
pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    (low..=high)
        .filter(|x| {
            let number = x.to_string();
            if number.len() % 2 == 0 {
                let digits = number
                    .chars()
                    .map(|digit| digit.to_digit(10).expect("digits = x.to_string"))
                    .collect::<Vec<_>>();
                digits.iter().take(number.len() / 2).sum::<u32>()
                    == digits.iter().skip(number.len() / 2).sum::<u32>()
            } else {
                false
            }
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_numbers() {
        assert_eq!(count_symmetric_integers(1, 100), 9);
    }

    #[test]
    fn big_numbers() {
        assert_eq!(count_symmetric_integers(1200, 1230), 4);
    }
}
