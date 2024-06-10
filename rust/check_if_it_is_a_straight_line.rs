//! <https://leetcode.com/problems/check-if-it-is-a-straight-line/>
pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    let y: Vec<i32> = coordinates.iter().map(|coordinate| coordinate[1]).collect();
    if y.iter().min() == y.iter().max() {
        return true;
    }
    let mut res = None;
    for (i, coordinate) in coordinates.iter().enumerate().skip(1) {
        let gradient = (coordinate[0] - coordinates[i - 1][0]) as f64
            / (coordinate[1] - coordinates[i - 1][1]) as f64;
        if let Some(res) = res {
            if res != gradient {
                return false;
            }
        } else {
            res = Some(gradient);
        }
    }
    true
}

fn main() {
    assert!(check_straight_line(vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
    ]),);
    assert!(!check_straight_line(vec![
        vec![1, 1],
        vec![2, 2],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![7, 7],
    ]),);
}
