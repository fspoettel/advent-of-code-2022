use advent_of_code::helpers::{
    grid_utils::{Direction, Point},
    sparse_grid::SparseGrid,
};
use std::cmp;

type Input = Vec<Step>;
type Step = (Direction, usize);

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            let (dir_str, moves) = l.split_once(' ')?;
            let steps = moves.parse().ok()?;
            let step = match dir_str {
                "R" => (Direction::East, steps),
                "U" => (Direction::North, steps),
                "L" => (Direction::West, steps),
                "D" => (Direction::South, steps),
                _ => unreachable!(),
            };
            Some(step)
        })
        .collect()
}

fn move_knot(head: &Point, tail: &Point) -> Point {
    Point {
        x: tail.x + cmp::max(cmp::min(head.x - tail.x, 1), -1),
        y: tail.y + cmp::max(cmp::min(head.y - tail.y, 1), -1),
    }
}

fn move_rope(steps: &Input, knot_count: usize) -> usize {
    // each knot starts at (0, 0).
    let mut knots: Vec<Point> = (0..knot_count).map(|_| Point { x: 0, y: 0 }).collect();

    let mut tail_grid = SparseGrid::default();
    tail_grid.insert(Point { x: 0, y: 0 }, true);

    steps.iter().for_each(|(dir, steps)| {
        for _ in 0..*steps {
            knots[0] = knots[0].get_neighbour(dir, 1);

            for i in 0..(knot_count - 1) {
                let head = &knots[i];
                let tail = &knots[i + 1];
                let tail_moves = head.chebyshev_distance(tail) > 1;

                if tail_moves {
                    knots[i + 1] = move_knot(head, tail);
                    if i == knot_count - 2 {
                        tail_grid.insert(knots[i + 1].clone(), true)
                    }
                }
            }
        }
    });

    tail_grid.points.len()
}

pub fn part_one(steps: Input) -> Option<usize> {
    Some(move_rope(&steps, 2))
}

pub fn part_two(steps: Input) -> Option<usize> {
    Some(move_rope(&steps, 10))
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        let result = part_one(parse(&input.split("\n\n").next().unwrap()));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        let result = part_two(parse(&input.split("\n\n").last().unwrap()));
        assert_eq!(result, Some(36));
    }
}
