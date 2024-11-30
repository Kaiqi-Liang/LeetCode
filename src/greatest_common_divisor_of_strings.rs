//! <https://leetcode.com/problems/greatest-common-divisor-of-strings/>
fn divides(divisor: &str, dividend: &str) -> bool {
    if dividend.len() % divisor.len() != 0 {
        return false;
    }
    (0..dividend.len() / divisor.len())
        .all(|i| *divisor == dividend[i * divisor.len()..(i + 1) * divisor.len()])
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let (shorter, longer) = if str1.len() < str2.len() {
        (str1, str2)
    } else {
        (str2, str1)
    };
    for i in (1..=shorter.len()).rev() {
        let divisor = &shorter[0..i];
        if divides(divisor, &shorter) && divides(divisor, &longer) {
            return divisor.to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entire_shorter_string() {
        assert_eq!(
            gcd_of_strings(String::from("ABCABC"), String::from("ABC")),
            String::from("ABC"),
        );
    }

    #[test]
    fn half_of_shorter_string() {
        assert_eq!(
            gcd_of_strings(String::from("ABABAB"), String::from("ABAB")),
            String::from("AB"),
        );
    }

    #[test]
    fn no_gcd() {
        assert_eq!(
            gcd_of_strings(String::from("LEET"), String::from("CODE")),
            String::new(),
        );
    }
}
