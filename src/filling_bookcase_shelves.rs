//! <https://leetcode.com/problems/filling-bookcase-shelves/>
pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut opt = vec![books[0][1]];
    for i in 1..books.len() {
        let mut remaining_width = shelf_width - books[i][0];
        let mut max_height = books[i][1];
        opt.push(opt[i - 1] + max_height);
        for j in (0..i).rev() {
            if books[j][0] <= remaining_width {
                max_height = max_height.max(books[j][1]);
                opt[i] = opt[i].min(max_height + if j > 0 { opt[j - 1] } else { 0 });
                remaining_width -= books[j][0];
            } else {
                break;
            }
        }
    }
    opt[books.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            min_height_shelves(
                vec![
                    vec![1, 1],
                    vec![2, 3],
                    vec![2, 3],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 2],
                ],
                4,
            ),
            6,
        );
    }

    #[test]
    fn all_on_one_shelf() {
        assert_eq!(
            min_height_shelves(vec![vec![1, 3], vec![2, 4], vec![3, 2]], 6),
            4,
        );
    }
}
