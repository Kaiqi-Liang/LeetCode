//! https://leetcode.com/problems/number-of-valid-words-in-a-sentence/
pub fn count_valid_words(sentence: String) -> i32 {
    sentence
        .split_whitespace()
        .filter(|word| {
            (match word.find(|c| c == ',' || c == '!' || c == '.') {
                Some(i) => i == word.len() - 1,
                None => true,
            }) && (match word.matches('-').count() {
                0 => true,
                1 => {
                    let i = word.find('-').unwrap();
                    let is_lowercase = |i| word.chars().nth(i).unwrap().is_ascii_lowercase();
                    i > 0 && i < word.len() - 1 && is_lowercase(i - 1) && is_lowercase(i + 1)
                }
                _ => false,
            }) && !word.chars().any(|c| c.is_ascii_digit())
        })
        .count() as i32
}

fn main() {
    assert_eq!(count_valid_words(String::from("-")), 0);
    assert_eq!(count_valid_words(String::from("c-t -and dog-")), 1);
    assert_eq!(count_valid_words(String::from("cat and  dog")), 3);
    assert_eq!(count_valid_words(String::from("cat and  dog")), 3);
    assert_eq!(count_valid_words(String::from("cat, an!d\n.dog")), 1);
    assert_eq!(count_valid_words(String::from("!this  1-s b8d!")), 0);
    assert_eq!(
        count_valid_words(String::from("alice and  bob are playing stone-game10")),
        5
    );
}
