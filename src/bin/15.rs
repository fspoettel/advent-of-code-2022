use std::cmp;

use advent_of_code::Point;
use hashbrown::HashSet;

#[derive(Clone, Debug)]
pub struct Sensor {
    at: Point,
    closest_beacon: Point,
}

type Input = Vec<Sensor>;

fn parse_point(s: &str) -> Option<Point> {
    let (_, point_str) = s.split_once("at ")?;
    let (x_str, y_str) = point_str.split_once(", ")?;
    let (_, x) = x_str.split_once('=')?;
    let (_, y) = y_str.split_once('=')?;
    Some(Point {
        x: x.parse().ok()?,
        y: y.parse().ok()?,
    })
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            let (sensor_str, beacon_str) = l.split_once(':')?;
            Some(Sensor {
                at: parse_point(sensor_str)?,
                closest_beacon: parse_point(beacon_str)?,
            })
        })
        .collect()
}

pub fn part_one(sensors: Input) -> Option<usize> {
    let target_row = if cfg!(test) { 10 } else { 2000000 };

    let mut found = 0;

    // optimization: track all beacons on `target-row` and subtract from final count.
    let mut beacons = HashSet::new();
    // optimization: track only sensors that can see `target_row`.
    let mut candidates = vec![];
    // optimization: track the min & max possible y on target row to reduce search space.
    // space is still a bit wider than need be, possible to optimize?
    let mut min_y = 0;
    let mut max_y = 0;

    for sensor in sensors.iter() {
        let max_distance = sensor.at.manhattan_distance(&sensor.closest_beacon);
        let y_max = sensor.at.y + max_distance;
        let y_min = sensor.at.y - max_distance;
        max_y = cmp::max(max_y, y_max);
        min_y = cmp::min(min_y, y_min);

        if y_max >= target_row && y_min <= target_row {
            candidates.push(sensor.clone());
            if sensor.closest_beacon.y == target_row {
                beacons.insert(sensor.closest_beacon.clone());
            }
        }
    }

    for x in min_y..max_y {
        for sensor in candidates.iter() {
            if sensor.at.manhattan_distance(&Point { x, y: target_row })
                <= sensor.at.manhattan_distance(&sensor.closest_beacon)
            {
                found += 1;
                break;
            }
        }
    }

    Some(found - beacons.len())
}

pub fn part_two(_: Input) -> Option<u32> {
    None
}

advent_of_code::main!(15);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 15)));
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 15)));
        assert_eq!(result, None);
    }
}
