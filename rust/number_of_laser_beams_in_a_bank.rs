//! <https://leetcode.com/problems/number-of-laser-beams-in-a-bank/>
pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let bank: Vec<_> = bank
        .into_iter()
        .filter_map(|row| {
            let count = row.matches('1').count();
            if count > 0 {
                Some(count)
            } else {
                None
            }
        })
        .collect();
    let mut total = 0;
    for (i, row) in bank.iter().enumerate().skip(1) {
        total += bank[i - 1] * row;
    }
    total as _
}

fn main() {
    assert_eq!(
        number_of_beams(vec![
            String::from("011001"),
            String::from("000000"),
            String::from("010100"),
            String::from("001000"),
        ]),
        8,
    );
    assert_eq!(
        number_of_beams(vec![
            String::from("000"),
            String::from("111"),
            String::from("000"),
        ]),
        0,
    );
}
