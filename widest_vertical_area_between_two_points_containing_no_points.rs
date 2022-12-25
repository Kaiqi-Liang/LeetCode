//! <https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/>
pub fn max_width_of_vetical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut points: Vec<&i32> = points
        .iter()
        .map(|point| {
            point
                .first()
                .expect("The inner Vec is guaranteed to be of size 2")
        })
        .collect();
    points.sort();
    let mut max = 0;
    for (i, point) in points.iter().enumerate().skip(1) {
        max = max.max(*point - points[i - 1]);
    }
    max
}

fn main() {
    assert_eq!(
        max_width_of_vetical_area(vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]]),
        1,
    );
    assert_eq!(
        max_width_of_vetical_area(vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8]
        ]),
        3,
    );
}
