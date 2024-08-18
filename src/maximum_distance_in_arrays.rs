//! <https://leetcode.com/problems/maximum-distance-in-arrays/>
macro_rules! get_min_max {
    ($array:expr) => {
        (get_min_max!($array, min), get_min_max!($array, max))
    };
    ($array:expr, $position:ident) => {
        $array.iter().$position().expect("arrays[i].len() >= 1")
    };
}
pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let (mut min_so_far, mut max_so_far) = get_min_max!(arrays.first().expect("arrays.len() >= 2"));
    let mut res = 0;
    for array in arrays.iter().skip(1) {
        let (curr_min, curr_max) = get_min_max!(array);
        res = res.max(curr_max - min_so_far);
        res = res.max(max_so_far - curr_min);
        min_so_far = min_so_far.min(curr_min);
        max_so_far = max_so_far.max(curr_max);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]),
            4,
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(max_distance(vec![vec![1], vec![1]]), 0);
    }
}
