/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */
#[derive(Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct SimpleGrid<T> {
    pub max_x: usize,
    pub max_y: usize,
    pub data: Vec<Vec<T>>,
}

impl<T> SimpleGrid<T> {
    pub fn from_str(input: &str, parse: &dyn Fn(char) -> T) -> Self {
        // map lines into a nested list, applying parser to each item.
        let data: Vec<Vec<T>> = input
            .trim()
            .lines()
            .map(|l| l.chars().map(parse).collect())
            .collect();

        SimpleGrid {
            max_x: data[0].len() - 1,
            max_y: data.len() - 1,
            data,
        }
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                points.push(Point { x, y })
            }
        }

        points
    }

    pub fn get(&self, point: &Point) -> &T {
        &self.data[point.y][point.x]
    }

    pub fn is_edge(&self, point: &Point) -> bool {
        point.x == 0 || point.y == 0 || point.x == self.max_x || point.y == self.max_y
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

pub struct WalkingIterator<'a, T> {
    current: Point,
    direction: &'a Direction,
    grid: &'a SimpleGrid<T>,
}

impl<T> Iterator for WalkingIterator<'_, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next_point = match self.direction {
            Direction::North => Point {
                x: self.current.x,
                y: self.current.y.checked_sub(1)?,
            },
            Direction::West => Point {
                x: self.current.x.checked_sub(1)?,
                y: self.current.y,
            },
            Direction::South => Point {
                x: self.current.x,
                y: self.current.y + 1,
            },
            Direction::East => Point {
                x: self.current.x + 1,
                y: self.current.y,
            },
        };

        if next_point.x > self.grid.max_x || next_point.y > self.grid.max_y {
            None
        } else {
            self.current = next_point;
            Some(self.current.clone())
        }
    }
}
