//! <https://leetcode.com/problems/beautiful-arrangement-ii/>
pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
    let mut answer = vec![1];
    for i in 0..k {
        answer.push(*answer.last().expect("answer.len() > 0") + (k - i) * (-1_i32).pow(i as u32));
    }
    for i in k + 2..=n {
        answer.push(i);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increasing() {
        assert_eq!(construct_array(3, 1), vec![1, 2, 3]);
    }

    #[test]
    fn interleaving() {
        assert_eq!(construct_array(3, 2), vec![1, 3, 2]);
    }

    #[test]
    fn first_interleaving_then_increasing() {
        assert_eq!(construct_array(10, 5), vec![1, 6, 2, 5, 3, 4, 7, 8, 9, 10]);
    }
}
