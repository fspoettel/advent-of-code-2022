use std::cmp;

#[derive(Clone)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn chebyshev_distance(&self, other: &Point) -> isize {
        cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    /// Get point x steps away in a given direction.
    #[inline(always)]
    pub fn get_neighbour(&self, direction: &Direction, steps: isize) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - steps,
            },
            Direction::NorthEast => Self {
                x: self.x + steps,
                y: self.y - steps,
            },
            Direction::East => Self {
                x: self.x + steps,
                y: self.y,
            },
            Direction::SouthEast => Self {
                x: self.x + steps,
                y: self.y + steps,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + steps,
            },
            Direction::SouthWest => Self {
                x: self.x - steps,
                y: self.y + steps,
            },
            Direction::West => Self {
                x: self.x - steps,
                y: self.y,
            },
            Direction::NorthWest => Self {
                x: self.x - steps,
                y: self.y - steps,
            },
        }
    }
}
