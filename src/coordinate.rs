#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

pub const NEIGHBOURS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Coordinate {
    pub fn neighbours(self, width: usize, height: usize) -> impl Iterator<Item = Coordinate> {
        NEIGHBOURS.iter().filter_map(move |(delta_x, delta_y)| {
            let (new_x, new_y) = (delta_x + self.x as i32, delta_y + self.y as i32);
            if new_x >= 0 && new_y >= 0 && new_x < width as i32 && new_y < height as i32 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                Some(Coordinate { x: new_x, y: new_y })
            } else {
                None
            }
        })
    }
}
