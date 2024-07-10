//! <https://leetcode.com/problems/crawler-log-folder/>
pub fn min_operations(logs: Vec<String>) -> i32 {
	let mut levels = 0;
	for log in logs {
		match log.as_str() {
			"./" => (),
			"../" => levels = 0.max(levels - 1),
			_ => levels += 1
		}
	}
    levels
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
