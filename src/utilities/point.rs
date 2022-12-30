pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn manhattan(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub const ORIGIN: Point = Point { x: 0, y: 0 };
