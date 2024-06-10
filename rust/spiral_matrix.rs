//! <https://leetcode.com/problems/spiral-matrix/>
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn get_perimeter(matrix: &[Vec<i32>], mut res: Vec<i32>, size: usize) -> Vec<i32> {
    if res.len() == size {
        return res;
    }

    let height: i32 = matrix.len() as _;
    let width: i32 = matrix[0].len() as _;

    let mut direction = if width > 1 {
        Direction::Right
    } else {
        Direction::Down
    };

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let perimeter = if width == 1 {
        height
    } else if height == 1 {
        width
    } else {
        1.max((width + height) * 2 - 4) // width > 1 && height > 1
    };

    for _ in 0..perimeter {
        res.push(matrix[i as usize][j as usize]);

        match direction {
            Direction::Right => {
                j += 1;
                if j == width - 1 {
                    direction = Direction::Down;
                }
            }
            Direction::Down => {
                i += 1;
                if i == height - 1 {
                    direction = Direction::Left;
                }
            }
            Direction::Left => {
                j -= 1;
                if j == 0 {
                    direction = Direction::Up;
                }
            }
            Direction::Up => {
                i -= 1;
                if i == 0 {
                    direction = Direction::Right;
                }
            }
        }
    }

    if height == 1 && width == 1 {
        res
    } else if height == 1 {
        get_perimeter(&[matrix[0][1..matrix[0].len() - 1].to_vec()], res, size)
    } else if width == 1 {
        get_perimeter(&matrix[1..matrix.len() - 1], res, size)
    } else {
        get_perimeter(
            &matrix[1..matrix.len() - 1]
                .iter()
                .map(|inner| inner[1..inner.len() - 1].to_vec())
                .collect::<Vec<Vec<i32>>>(),
            res,
            size,
        )
    }
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let height = matrix.len();
    let width = matrix[0].len();
    let size = width * height;
    get_perimeter(&matrix, Vec::with_capacity(size), size)
}

fn main() {
    assert_eq!(
        spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
    );
    assert_eq!(spiral_order(vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 4, 3]);
    assert_eq!(
        spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]),
        vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10],
    );
    assert_eq!(
        spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
    );
    assert_eq!(spiral_order(vec![vec![3], vec![2]]), vec![3, 2]);
    assert_eq!(spiral_order(vec![vec![6, 9, 7]]), vec![6, 9, 7]);
}
