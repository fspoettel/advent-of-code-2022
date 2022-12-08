/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */
use hashbrown::HashMap;
use std::slice::Iter;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone)]
pub struct Grid<T> {
    pub max_x: isize,
    pub max_y: isize,
    pub min_x: isize,
    pub min_y: isize,
    pub points: HashMap<Point, T>,
}

impl<T> Grid<T> {
    pub fn from_str(
        input: &str,
        parse: &dyn Fn(char) -> T,
        filter: Option<&dyn Fn(char) -> bool>,
    ) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut points = HashMap::new();

        input.trim().lines().enumerate().for_each(|(y, l)| {
            if y > max_y {
                max_y = y;
            }

            l.chars().enumerate().for_each(|(x, c)| {
                if x > max_x {
                    max_x = x;
                }

                if filter.is_none() || !filter.unwrap()(c) {
                    points.insert(
                        Point {
                            x: x as isize,
                            y: y as isize,
                        },
                        parse(c),
                    );
                }
            });
        });

        Grid {
            points,
            min_x: 0,
            min_y: 0,
            max_x: max_x as isize,
            max_y: max_y as isize,
        }
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        self.points.get(point)
    }

    pub fn is_edge(&self, point: &Point) -> bool {
        point.x == self.min_x
            || point.y == self.min_y
            || point.x == self.max_x
            || point.y == self.max_y
    }

    pub fn is_inside(&self, point: &Point) -> bool {
        point.x >= self.min_x
            && point.y >= self.min_y
            && point.x <= self.max_x
            && point.y <= self.max_y
    }

    pub fn walk<'a>(&'a self, current: &'a Point, direction: &'a Direction) -> WalkingIterator<T> {
        WalkingIterator {
            current: current.clone(),
            grid: self,
            direction,
        }
    }
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        DIRECTIONS.iter()
    }
}

pub struct WalkingIterator<'a, T> {
    current: Point,
    direction: &'a Direction,
    grid: &'a Grid<T>,
}

impl<T> Iterator for WalkingIterator<'_, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let x = match self.direction {
            Direction::West => self.current.x - 1,
            Direction::East => self.current.x + 1,
            _ => self.current.x,
        };

        let y = match self.direction {
            Direction::North => self.current.y - 1,
            Direction::South => self.current.y + 1,
            _ => self.current.y,
        };

        self.current = Point { x, y };

        if self.grid.is_inside(&self.current) {
            Some(self.current.clone())
        } else {
            None
        }
    }
}
