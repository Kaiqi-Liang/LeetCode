//! <https://leetcode.com/problems/valid-boomerang/>
pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
    if points[0] == points[1] || points[1] == points[2] {
        return false;
    }
    let delta_x1: f64 = (points[1][0] - points[0][0]).into();
    let delta_x2: f64 = (points[2][0] - points[1][0]).into();
    let delta_y1: f64 = (points[1][1] - points[0][1]).into();
    let delta_y2: f64 = (points[2][1] - points[1][1]).into();
    if delta_x1 == 0. && delta_x2 == 0. || delta_y1 == 0. && delta_y2 == 0. {
        return false;
    }
    delta_y1 / delta_x1 != delta_y2 / delta_x2
}

fn main() {
    assert!(is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]));
    assert!(is_boomerang(vec![vec![0, 1], vec![0, 2], vec![1, 2]]));
    assert!(!is_boomerang(vec![vec![0, 0], vec![1, 1], vec![1, 1]]));
    assert!(!is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]));
}
