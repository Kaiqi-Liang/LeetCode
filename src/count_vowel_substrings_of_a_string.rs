//! <https://leetcode.com/problems/count-vowel-substrings-of-a-string/>
use std::collections::HashSet;
pub fn count_vowel_substrings(word: String) -> i32 {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let mut count = 0;
    for i in 0..word.len() {
        let mut substring = HashSet::new();
        for j in i..word.len() {
            let curr = word.chars().nth(j).unwrap();
            if vowels.contains(&curr) {
                substring.insert(curr);
                if substring.len() == 5 {
                    count += 1;
                }
            } else {
                break;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(count_vowel_substrings(String::from("aeiouu")), 2);
    }

    #[test]
    fn no_vowel_substring() {
        assert_eq!(count_vowel_substrings(String::from("unicornarihan")), 0);
    }

    #[test]
    fn multiple_overlapping_substrings() {
        assert_eq!(count_vowel_substrings(String::from("cuaieuouac")), 7);
    }
}
