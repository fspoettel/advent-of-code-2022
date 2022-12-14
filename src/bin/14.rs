use std::cmp;

use advent_of_code::{Direction, Point, SparseGrid};
use itertools::Itertools;

type Grid = SparseGrid<PointType>;
type Input<'a> = (Grid, isize);

static SOURCE: Point = Point { x: 500, y: 0 };

#[derive(Clone, Eq, PartialEq)]
pub enum PointType {
    Wall,
    Sand,
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .fold((SparseGrid::default(), 0_isize), |mut acc, l| {
            l.split(" -> ")
                .filter_map(|segment| {
                    let (x, y) = segment.split_once(',')?;
                    Some(Point {
                        x: x.parse().ok()?,
                        y: y.parse().ok()?,
                    })
                })
                .tuple_windows()
                .for_each(|(a, b)| {
                    for point in a.line_to(&b) {
                        acc.1 = cmp::max(acc.1, point.y);
                        acc.0.insert(point, PointType::Wall);
                    }
                });

            acc
        })
}

fn next_pos(grid: &Grid, point: &Point) -> Option<Point> {
    let south = point.get_neighbour(&Direction::South, 1);
    if grid.get(&south).is_none() {
        return Some(south);
    }

    let south_east = point.get_neighbour(&Direction::SouthEast, 1);
    if grid.get(&south_east).is_none() {
        return Some(south_east);
    }

    let south_west = point.get_neighbour(&Direction::SouthWest, 1);
    if grid.get(&south_west).is_none() {
        return Some(south_west);
    }

    None
}

fn count_sand(grid: &Grid) -> usize {
    grid.points
        .values()
        .filter(|x| **x == PointType::Sand)
        .count()
}

pub fn part_one((mut grid, floor): Input) -> Option<usize> {
    let mut current_sand = SOURCE.clone();

    loop {
        match next_pos(&grid, &current_sand) {
            Some(p) => current_sand = p,
            None => {
                grid.insert(current_sand, PointType::Sand);
                current_sand = SOURCE.clone();
            }
        }

        if current_sand.y >= floor {
            break;
        }
    }

    Some(count_sand(&grid))
}

pub fn part_two((mut grid, floor): Input) -> Option<usize> {
    let floor = floor + 2;
    let mut current_sand = SOURCE.clone();

    loop {
        if let Some(next_sand) = next_pos(&grid, &current_sand) {
            if next_sand.y < floor {
                current_sand = next_sand;
                continue;
            }
        } else if current_sand == SOURCE {
            grid.insert(current_sand, PointType::Sand);
            break;
        }

        grid.insert(current_sand.clone(), PointType::Sand);
        current_sand = SOURCE.clone();
    }

    Some(count_sand(&grid))
}

advent_of_code::main!(14);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 14)));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 14)));
        assert_eq!(result, Some(93));
    }
}
