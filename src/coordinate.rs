#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Default)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

pub const IMMEDIATE_NEIGHBOURS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub const DIAGONAL_NEIGHBOURS: [(i32, i32); 4] = [(-1, -1), (1, 1), (-1, 1), (1, -1)];

impl From<Coordinate> for (i32, i32) {
    fn from(coordinate: Coordinate) -> Self {
        (coordinate.x as i32, coordinate.y as i32)
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as usize,
            y: value.1 as usize,
        }
    }
}

impl Coordinate {
    pub fn neighbours(
        self,
        width: usize,
        height: usize,
        neighbours: &[(i32, i32)],
    ) -> impl Iterator<Item = Coordinate> + '_ {
        neighbours.iter().filter_map(move |(delta_x, delta_y)| {
            let (new_x, new_y) = (delta_x + self.x as i32, delta_y + self.y as i32);
            if new_x >= 0 && new_y >= 0 && new_x < width as i32 && new_y < height as i32 {
                Some((new_x, new_y).into())
            } else {
                None
            }
        })
    }
}
