use crate::helpers::grid_utils::{Direction, Point};
use crate::helpers::simple_grid::SimpleGrid;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
