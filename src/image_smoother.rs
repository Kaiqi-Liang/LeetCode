//! <https://leetcode.com/problems/image-smoother/>
use crate::coordinate::{Coordinate, DIAGONAL_NEIGHBOURS, IMMEDIATE_NEIGHBOURS};
pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut smoothed_img = img.clone();
    let width = img.len();
    let height = img.first().expect("img[i].len() >= 1").len();
    #[allow(clippy::needless_range_loop)]
    for x in 0..width {
        for y in 0..height {
            let curr_coord = Coordinate { x, y };
            let neighbours = curr_coord
                .neighbours(
                    width,
                    height,
                    &[
                        &IMMEDIATE_NEIGHBOURS[..],
                        &DIAGONAL_NEIGHBOURS[..],
                        &[Coordinate::default().into()],
                    ]
                    .concat(),
                )
                .collect::<Vec<_>>();
            let sum = neighbours
                .iter()
                .map(|Coordinate { x, y }| img[*x][*y])
                .sum::<i32>();
            smoothed_img[x][y] = sum / neighbours.len() as i32;
        }
    }
    smoothed_img
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100],
            ]),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137],
            ],
        );
    }

    #[test]
    fn all_zeros() {
        assert_eq!(
            image_smoother(vec![vec![1; 3], vec![1, 0, 1], vec![1; 3]]),
            vec![vec![0; 3]; 3],
        );
    }
}
