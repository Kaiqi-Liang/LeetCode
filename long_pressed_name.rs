//! <https://leetcode.com/problems/long-pressed-name/>
pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    'outer: while i < name.len() {
        let ch = name.chars().nth(i).expect("i < name.len()");
        let mut repetition = 1;
        while i < name.len() - 1 && ch == name.chars().nth(i + 1).expect("i < name.len() - 1") {
            i += 1;
            repetition += 1;
        }
        let mut matched = 0;
        loop {
            if ch
                == match typed.chars().nth(j) {
                    Some(ch) => ch,
                    None => break 'outer,
                }
            {
                matched += 1;
            } else {
                break;
            }
            j += 1;
        }
        if matched < repetition {
            return false;
        }
        i += 1;
    }
    i == name.len() - 1
}

fn main() {
    assert!(!is_long_pressed_name(String::from("a"), String::from("b")));
    assert!(is_long_pressed_name(
        String::from("alex"),
        String::from("aaleex")
    ));
    assert!(is_long_pressed_name(
        String::from("leelee"),
        String::from("lleeelee")
    ));
    assert!(!is_long_pressed_name(
        String::from("saeed"),
        String::from("")
    ));
    assert!(!is_long_pressed_name(
        String::from("saeed"),
        String::from("ssaaedd")
    ));
}
