//! <https://leetcode.com/problems/relative-sort-array/>
use std::collections::HashMap;
pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let ordering = arr2
        .into_iter()
        .enumerate()
        .fold(HashMap::new(), |mut a, c| {
            a.insert(c.1, c.0);
            a
        });
    let mut elems_in_ordering = Vec::new();
    let mut elems_not_in_ordering = Vec::new();
    for elem in arr1 {
        (if ordering.contains_key(&elem) {
            &mut elems_in_ordering
        } else {
            &mut elems_not_in_ordering
        })
        .push(elem);
    }
    elems_in_ordering.sort_by(|a, b| ordering[a].cmp(&ordering[b]));
    elems_not_in_ordering.sort();
    elems_in_ordering
        .into_iter()
        .chain(elems_not_in_ordering)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }

    #[test]
    fn elems_not_in_ordering() {
        assert_eq!(
            relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
            vec![22, 28, 8, 6, 17, 44]
        );
    }
}
