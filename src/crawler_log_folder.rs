//! <https://leetcode.com/problems/crawler-log-folder/>
pub fn min_operations(logs: Vec<String>) -> i32 {
    logs.into_iter().fold(0, |a, c| match c.as_str() {
        "./" => a,
        "../" => 0.max(a - 1),
        _ => a + 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_levels_deep() {
        assert_eq!(
            min_operations(vec![
                String::from("d1/"),
                String::from("d2/"),
                String::from("../"),
                String::from("d21/"),
                String::from("./"),
            ]),
            2
        );
    }

    #[test]
    fn three_levels_deep() {
        assert_eq!(
            min_operations(vec![
                String::from("d1/"),
                String::from("d2/"),
                String::from("./"),
                String::from("d3/"),
                String::from("../"),
                String::from("d31/"),
            ]),
            3
        );
    }

    #[test]
    fn try_to_go_up_from_root() {
        assert_eq!(
            min_operations(vec![
                String::from("d1/"),
                String::from("../"),
                String::from("../"),
                String::from("../"),
            ]),
            0
        );
    }
}
