use super::grid_utils::{Direction, Point};

#[derive(Clone, Debug)]
pub struct SimpleGrid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<T>>,
}

/// Simple 2D grid that expects to hold a value at each point.
impl<T> SimpleGrid<T> {
    /// Create a grid holding items of type T from a string representation.
    /// Parser is called with (char, x, y).
    pub fn from_str(input: &str, parse: &mut dyn FnMut(char, usize, usize) -> T) -> Self {
        // map lines into a nested list, applying parser to each item.
        let data: Vec<Vec<T>> = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, l)| l.chars().enumerate().map(|(x, c)| parse(c, x, y)).collect())
            .collect();

        SimpleGrid {
            width: data[0].len(),
            height: data.len(),
            data,
        }
    }

    /// Get all points in grid.
    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                points.push(Point {
                    x: x as isize,
                    y: y as isize,
                })
            }
        }

        points
    }

    /// Get a reference to the value for a certain point in the grid.
    /// NOTE: unchecked.
    pub fn get(&self, point: &Point) -> &T {
        &self.data[point.y as usize][point.x as usize]
    }

    /// Check if a point is inside the grid.
    pub fn is_inside(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && (point.x as usize) < self.width
            && (point.y as usize) < self.height
    }

    /// Check if a point is on the boundary of the grid.
    pub fn is_boundary(&self, point: &Point) -> bool {
        point.x == 0
            || point.y == 0
            || point.x as usize == self.width - 1
            || point.y as usize == self.height - 1
    }

    /// Get an iterator that: starting at point `p`, walks to the edge of the grid in direction `d`.
    pub fn walk<'a>(&'a self, current: &'a Point, direction: &'a Direction) -> WalkingIterator<T> {
        WalkingIterator {
            current: current.clone(),
            grid: self,
            direction,
        }
    }

    /// Get a unique identifier for a point in this grid.
    pub fn id_for_point(&self, p: &Point) -> usize {
        p.x as usize + self.width * p.y as usize
    }

    /// Get underlying point for a unique idenfitier in this grid.
    pub fn point_for_id(&self, id: usize) -> Point {
        Point {
            x: (id % self.width) as isize,
            y: (id / self.width) as isize,
        }
    }
}

pub struct WalkingIterator<'a, T> {
    current: Point,
    direction: &'a Direction,
    grid: &'a SimpleGrid<T>,
}

impl<T> Iterator for WalkingIterator<'_, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next_point = self.current.get_neighbour(self.direction, 1);

        if next_point.x < 0
            || next_point.y < 0
            || next_point.x as usize > self.grid.width - 1
            || next_point.y as usize > self.grid.height - 1
        {
            None
        } else {
            self.current = next_point;
            Some(self.current.clone())
        }
    }
}
