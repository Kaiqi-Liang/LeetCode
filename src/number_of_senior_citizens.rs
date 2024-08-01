/// <https://leetcode.com/problems/number-of-senior-citizens/>
pub fn count_seniors(details: Vec<String>) -> i32 {
    details
        .into_iter()
        .filter(|detail| detail[11..13].parse::<i32>().unwrap_or(0) > 60)
        .count() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            count_seniors(vec![
                String::from("7868190130M7522"),
                String::from("5303914400F9211"),
                String::from("9273338290F4010"),
            ]),
            2,
        );
    }

    #[test]
    fn no_senior() {
        assert_eq!(
            count_seniors(vec![
                String::from("1313579440F2036"),
                String::from("1313579440F2036"),
            ]),
            0,
        );
    }
}
