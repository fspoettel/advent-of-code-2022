use advent_of_code::helpers::{
    grid_utils::Point, shortest_path::shortest_path, simple_grid::SimpleGrid,
};

type Input<'a> = (SimpleGrid<char>, Point, Point);

fn parse(input: &str) -> Input {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let grid: SimpleGrid<char> = SimpleGrid::from_str(input, &mut |c, x, y| match c {
        'S' => {
            start = Some(Point {
                x: x as isize,
                y: y as isize,
            });
            'a'
        }
        'E' => {
            end = Some(Point {
                x: x as isize,
                y: y as isize,
            });
            'z'
        }
        c => c,
    });

    (grid, start.unwrap(), end.unwrap())
}

fn find_path(grid: &SimpleGrid<char>, start: Vec<Point>, end: Point) -> Option<usize> {
    shortest_path(
        grid,
        &start,
        &end,
        |_| 1,
        |a, b| (*grid.get(b) as isize - *grid.get(a) as isize) < 2,
    )
}

pub fn part_one((grid, start, end): Input) -> Option<usize> {
    find_path(&grid, vec![start], end)
}

pub fn part_two((grid, _, end): Input) -> Option<usize> {
    let start_points: Vec<Point> = grid
        .points()
        .into_iter()
        .filter(|point| *grid.get(point) == 'a')
        .collect();
    find_path(&grid, start_points, end)
}

advent_of_code::main!(12);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 12)));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::read_file("examples", 12)));
        assert_eq!(result, Some(29));
    }
}
