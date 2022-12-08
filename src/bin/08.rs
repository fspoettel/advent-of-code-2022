use advent_of_code::helpers::{Direction, SimpleGrid};

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
            .filter(|a| {
                let size = grid.get(a);
                grid.is_edge(a)
                    || DIRECTIONS
                        .iter()
                        .any(|dir| grid.walk(a, dir).all(|b| grid.get(&b) < size))
            })
            .count(),
    )
}

pub fn part_two(grid: Input) -> Option<isize> {
    grid.points()
        .iter()
        .filter(|p| !grid.is_edge(p))
        .map(|a| {
            let size = grid.get(a);

            DIRECTIONS
                .iter()
                .map(|dir| {
                    let mut points = 0;
                    for b in grid.walk(a, dir) {
                        points += 1;
                        if grid.get(&b) >= size {
                            break;
                        }
                    }
                    points
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
