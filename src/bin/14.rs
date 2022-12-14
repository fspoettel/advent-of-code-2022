use advent_of_code::{Direction, Point, SparseGrid};
use itertools::Itertools;
use std::cmp;

type Grid = SparseGrid<PointType>;
type Input<'a> = (Grid, isize);

#[derive(Clone, Eq, PartialEq)]
pub enum PointType {
    Wall,
    Sand,
}

static SOURCE: Point = Point { x: 500, y: 0 };

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
    let mut current_path = vec![SOURCE.clone()];

    loop {
        let current_sand = &current_path[current_path.len() - 1];

        match next_pos(&grid, current_sand) {
            Some(next_sand) => {
                if current_sand.y >= floor {
                    break;
                } else {
                    current_path.push(next_sand);
                }
            }
            None => {
                grid.insert(current_sand.clone(), PointType::Sand);
                current_path.pop();
            }
        }
    }

    Some(count_sand(&grid))
}

pub fn part_two((mut grid, floor): Input) -> Option<usize> {
    let floor = floor + 2;
    let mut current_path = vec![SOURCE.clone()];

    loop {
        let current_sand = &current_path[current_path.len() - 1];

        if let Some(next_sand) = next_pos(&grid, current_sand) {
            if next_sand.y < floor {
                current_path.push(next_sand);
                continue;
            }
        }

        grid.insert(current_sand.clone(), PointType::Sand);
        if current_sand == &SOURCE {
            break;
        } else {
            current_path.pop();
        }
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
