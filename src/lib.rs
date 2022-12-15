use hashbrown::HashMap;
use std::{cmp, slice::Iter};

pub mod template;

#[derive(Clone, Debug)]
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

impl Direction {
    pub fn all() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ];

        DIRECTIONS.iter()
    }
}

/// A point in a 2D grid.
/// Uses `isize` to support use in sparse grids where point indexes may be negative.
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
                x: self.x - steps,
                y: self.y - steps,
            },
            Direction::East => Self {
                x: self.x - steps,
                y: self.y,
            },
            Direction::SouthEast => Self {
                x: self.x - steps,
                y: self.y + steps,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + steps,
            },
            Direction::SouthWest => Self {
                x: self.x + steps,
                y: self.y + steps,
            },
            Direction::West => Self {
                x: self.x + steps,
                y: self.y,
            },
            Direction::NorthWest => Self {
                x: self.x + steps,
                y: self.y - steps,
            },
        }
    }

    #[inline(always)]
    pub fn get_direction(&self, target: &Point) -> Option<Direction> {
        if self == target {
            return None;
        }

        let direction = if self.x == target.x {
            if self.y < target.y {
                Direction::South
            } else {
                Direction::North
            }
        } else if self.y == target.y {
            if self.x < target.x {
                Direction::West
            } else {
                Direction::East
            }
        } else if self.y < target.y {
            if self.x < target.x {
                Direction::NorthWest
            } else {
                Direction::NorthEast
            }
        } else if self.x < target.x {
            Direction::SouthWest
        } else {
            Direction::SouthEast
        };

        Some(direction)
    }

    pub fn line_to(&self, target: &Point) -> Vec<Point> {
        let mut points = vec![self.clone()];

        match self.get_direction(target) {
            None => points,
            Some(dir) => {
                let mut cursor = self.clone();
                loop {
                    cursor = cursor.get_neighbour(&dir, 1);
                    points.push(cursor.clone());
                    if &cursor == target {
                        break;
                    }
                }

                points
            }
        }
    }
}

/// Sparse grid where points may not exist at creation, or be negative.
#[derive(Clone, Debug)]
pub struct SparseGrid<T> {
    pub points: HashMap<Point, T>,
}

impl<T> SparseGrid<T> {
    pub fn get(&self, point: &Point) -> Option<&T> {
        self.points.get(point)
    }

    pub fn insert(&mut self, point: Point, data: T) {
        self.points.insert(point, data);
    }
}

impl<T> Default for SparseGrid<T> {
    fn default() -> Self {
        Self {
            points: HashMap::new(),
        }
    }
}

/// Simple 2D grid where each point maps to a value.
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

    /// Get a reference to the value for a certain point in the grid.
    /// NOTE: unchecked.
    pub fn get(&self, point: &Point) -> &T {
        &self.data[point.y as usize][point.x as usize]
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

    pub fn walk<'a>(&'a self, current: &'a Point, direction: &'a Direction) -> WalkingIterator<T> {
        WalkingIterator {
            current: current.clone(),
            grid: self,
            direction,
        }
    }
}

/// An iterator that: starting at point `p`, walks to the edge of the grid in direction `d`.
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

/// Implementation odf dijkstra's algorithm.
pub mod shortest_path {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    use crate::{Direction, Point, SimpleGrid};

    // while performing the search, track a sorted list of candidates (=state) to visit next on a priority queue.
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: usize,
    }

    /// the algorithm expects a `min-heap` priority queue as frontier.
    /// the default std. lib implementation is a `max-heap`, so the sort order needs to be flipped for state values.
    /// also adds a tie breaker based on position. see [rust docs](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#min-heap)
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    static DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    pub fn shortest_path<T>(
        grid: &SimpleGrid<T>,
        start_points: &[Point],
        end_point: &Point,
        calc_cost: impl Fn(&Point) -> usize,
        filter_neighbour: impl Fn(&Point, &Point) -> bool,
    ) -> Option<usize> {
        // dist[node] = current shortest distance from `start` to `node`.
        let mut dist: Vec<_> = (0..(grid.width * grid.height))
            .map(|_| usize::MAX)
            .collect();
        let mut frontier = BinaryHeap::new();

        // initialize each start point with a zero cost and push it to frontier.
        for start_point in start_points {
            let start_id = grid.id_for_point(start_point);
            dist[start_id] = 0;
            frontier.push(State {
                cost: 0,
                position: start_id,
            });
        }

        let end_id = grid.id_for_point(end_point);

        // examine the frontier starting with the lowest cost nodes.
        while let Some(State { cost, position }) = frontier.pop() {
            if position == end_id {
                return Some(cost);
            }

            // skip: there is a better path to this node already.
            if cost > dist[position] {
                continue;
            }

            let current = grid.point_for_id(position);

            let neighbours: Vec<Point> = DIRECTIONS
                .iter()
                .filter_map(|dir| {
                    let neighbour = current.get_neighbour(dir, 1);

                    if grid.is_inside(&neighbour) && filter_neighbour(&current, &neighbour) {
                        Some(neighbour)
                    } else {
                        None
                    }
                })
                .collect();

            // see if we can find a path with a lower cost than previous paths for any adjacent nodes.
            for neighbour in neighbours {
                let next = State {
                    cost: cost + calc_cost(&neighbour),
                    position: grid.id_for_point(&neighbour),
                };

                // if so, add it to the frontier and continue.
                if next.cost < dist[next.position] {
                    frontier.push(next);
                    dist[next.position] = next.cost;
                }
            }
        }

        None
    }
}
