//! <https://leetcode.com/problems/word-break/>
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut opt = vec![false; s.len() + 1];
    *opt.first_mut().unwrap() = true;
    for i in 1..=s.len() {
        for word in word_dict.iter() {
            if s[0..i].ends_with(word) && opt[i - word.len()] {
                opt[i] = true;
            }
        }
    }
    *opt.last().unwrap()
}

fn main() {
    assert!(!word_break(String::from("hi"), vec![]));
    assert!(word_break(
        String::from("leetcode"),
        vec![String::from("leet"), String::from("code")],
    ));
    assert!(word_break(
        String::from("applepenapple"),
        vec![String::from("apple"), String::from("pen")],
    ));
    assert!(!word_break(
        String::from("catsandog"),
        vec![
            String::from("cats"),
            String::from("dog"),
            String::from("sand"),
            String::from("and"),
            String::from("cat"),
        ],
    ));
}
