//! <https://leetcode.com/problems/longest-common-prefix/>
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let len = strs.iter().map(|s| s.len()).min().unwrap();
    let mut res = String::new();
    for index in 0..len {
        let curr = strs[0].chars().nth(index);
        if strs.iter().all(|s| s.chars().nth(index) == curr) {
            res.push(curr.unwrap());
        } else {
            break;
        }
    }
    res
}

fn main() {
    assert_eq!(
        longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ]),
        String::from("fl")
    );
    assert_eq!(
        longest_common_prefix(vec![String::from("")]),
        String::from("")
    );
}
