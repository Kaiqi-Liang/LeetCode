//! <https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/>
pub fn append_characters(s: String, t: String) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let s = Vec::from(s);
    let t = Vec::from(t);
    while i < s.len() && j < t.len() {
        if s[i] == t[j] {
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }
    (t.len() - j) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_subsequence() {
        assert_eq!(
            append_characters(String::from("coaching"), String::from("coding")),
            4
        );
    }

    #[test]
    fn already_a_subsequence() {
        assert_eq!(
            append_characters(String::from("coachding"), String::from("coding")),
            0
        );
        assert_eq!(
            append_characters(String::from("zcaobdcidnegf"), String::from("coding")),
            0
        );
    }

    #[test]
    fn append_none() {
        assert_eq!(
            append_characters(String::from("abcde"), String::from("a")),
            0
        );
    }

    #[test]
    fn append_all() {
        assert_eq!(
            append_characters(String::from("z"), String::from("abcde")),
            5
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            append_characters(String::from(""), String::from("abcde")),
            5
        );
        assert_eq!(
            append_characters(String::from("abcde"), String::from("")),
            0
        );
        assert_eq!(append_characters(String::from(""), String::from("")), 0);
    }
}
