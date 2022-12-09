use advent_of_code::helpers::{grid_utils::Direction, simple_grid::SimpleGrid};

type Input<'a> = SimpleGrid<u32>;

static DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn parse(input: &str) -> Input {
    SimpleGrid::from_str(input, &|x| x.to_digit(10).unwrap())
}

pub fn part_one(grid: Input) -> Option<usize> {
    Some(
        grid.points()
            .iter()
            .filter(|point| {
                if grid.is_edge(point) {
                    true
                } else {
                    let size = grid.get(point);

                    DIRECTIONS.iter().any(|dir| {
                        // walk line of sight until one is larger than tree.
                        grid.walk(point, dir)
                            .all(|point_b| grid.get(&point_b) < size)
                    })
                }
            })
            .count(),
    )
}

pub fn part_two(grid: Input) -> Option<isize> {
    grid.points()
        .iter()
        .filter(|point| !grid.is_edge(point))
        .map(|point| {
            let size = grid.get(point);

            DIRECTIONS
                .iter()
                .map(|dir| {
                    let mut in_line_of_sight = 0;
                    // walk line of sight until one is larger than tree.
                    for point_b in grid.walk(point, dir) {
                        in_line_of_sight += 1;
                        if grid.get(&point_b) >= size {
                            break;
                        }
                    }
                    in_line_of_sight
                })
                .product()
        })
        .max()
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 8)));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::read_file("examples", 8)));
        assert_eq!(result, Some(8));
    }
}
