use super::{orientation::Orientation, point::Point};

use std::cmp::{max, min};

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn length(&self) -> usize {
        self.start.manhattan(&self.end)
    }

    pub fn orientation(&self, point: &Point) -> Orientation {
        let orientation = (self.end.y - self.start.y) * (point.x - self.end.x)
            - (self.end.x - self.start.x) * (point.y - self.end.y);

        if orientation == 0 {
            return Orientation::Collinear;
        }

        if orientation > 0 {
            return Orientation::Clockwise;
        }

        Orientation::CounterClockwise
    }

    pub fn on_line(&self, point: &Point) -> bool {
        point.x <= max(self.start.x, self.end.x)
            && point.x >= min(self.start.x, self.end.x)
            && point.y <= max(self.start.y, self.end.y)
            && point.y >= min(self.start.y, self.end.y)
    }

    pub fn intersect(&self, other: &Line) -> bool {
        let orientation_one = self.orientation(&other.start);
        let orientation_two = self.orientation(&other.end);
        let orientation_three = other.orientation(&self.start);
        let orientation_four = other.orientation(&self.end);

        // General case
        if orientation_one != orientation_two && orientation_three != orientation_four {
            return true;
        }

        // Special Cases
        if orientation_one == Orientation::Collinear && self.on_line(&other.start) {
            return true;
        }

        if orientation_two == Orientation::Collinear && self.on_line(&other.end) {
            return true;
        }

        if orientation_three == Orientation::Collinear && other.on_line(&self.start) {
            return true;
        }

        if orientation_four == Orientation::Collinear && other.on_line(&self.end) {
            return true;
        }

        false
    }
}
