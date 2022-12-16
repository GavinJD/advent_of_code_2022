use std::{
    cmp::{max, min},
    fmt,
    ops::RangeInclusive,
};

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_15.txt");
    println!(
        "Part 1: {}",
        columns_without_beacon_optimized(input, 2000000)?
    );
    println!(
        "Part 2: {}",
        search_for_distress_beacon(input, 0, 4000000).unwrap()
    );
    Ok(())
}

// fn columns_without_beacon(input: &str, y: isize) -> Result<u64> {
//     let sensors = parse_sensors(input)?;
//
//     let (mut left_boundary, mut right_boundary) =
//         match sensors.iter().map(|s| s.nearest_beacon.x).minmax() {
//             itertools::MinMaxResult::NoElements => (0, 0),
//             itertools::MinMaxResult::OneElement(i) => (i, i),
//             itertools::MinMaxResult::MinMax(i, j) => (i, j),
//         };
//     let wiggle_room = sensors
//         .iter()
//         .map(|s| s.nearest_beacon_distance)
//         .max()
//         .unwrap_or(0);
//     left_boundary -= wiggle_room as isize;
//     right_boundary += wiggle_room as isize;
//     // println!("{left_boundary}, {right_boundary}, {wiggle_room}");
//
//     let mut covered_by_beacon = 0;
//     for x in left_boundary..=right_boundary {
//         let current_point = Point { x, y };
//         if sensors
//             .iter()
//             .any(|s| s.nearest_beacon == current_point || s.position == current_point)
//         {
//             continue;
//         }
//         if sensors
//             .iter()
//             .any(|s| s.position.distance_to(&current_point) <= s.nearest_beacon_distance)
//         {
//             covered_by_beacon += 1;
//             // println!("Covered: ({}, {})", current_point.x, current_point.y);
//         }
//     }
//
//     // let bottom_boundary = sensors
//     //     .iter()
//     //     .map(|s| s.nearest_beacon.y)
//     //     .max()
//     //     .unwrap_or(0);
//     // println!("{left_boundary} -> {right_boundary}");
//     // for y in 0..=bottom_boundary {
//     //     print!("{:3} : ", y);
//     //     for x in left_boundary..=right_boundary {
//     //         let curr = Point { x, y };
//     //         print!(
//     //             "{}",
//     //             if sensors.iter().any(|s| s.position == curr) {
//     //                 "S"
//     //             } else if sensors.iter().any(|s| s.nearest_beacon == curr) {
//     //                 "B"
//     //             } else if sensors
//     //                 .iter()
//     //                 .any(|s| s.position.distance_to(&curr) <= s.nearest_beacon_distance)
//     //             {
//     //                 "#"
//     //             } else {
//     //                 "."
//     //             }
//     //         )
//     //     }
//     //     println!();
//     // }
//
//     Ok(covered_by_beacon)
// }

fn columns_without_beacon_optimized(input: &str, y: isize) -> Result<isize> {
    let sensors = parse_sensors(input)?;

    let (mut left_boundary, mut right_boundary) =
        match sensors.iter().map(|s| s.nearest_beacon.x).minmax() {
            itertools::MinMaxResult::NoElements => (0, 0),
            itertools::MinMaxResult::OneElement(i) => (i, i),
            itertools::MinMaxResult::MinMax(i, j) => (i, j),
        };
    let wiggle_room = sensors
        .iter()
        .map(|s| s.nearest_beacon_distance)
        .max()
        .unwrap_or(0);
    left_boundary -= wiggle_room as isize;
    right_boundary += wiggle_room as isize;
    // println!("{left_boundary}, {right_boundary}, {wiggle_room}");
    // let (left_boundary, right_boundary) = (-10_000_000, 20_000_000);

    let covered_by_beacon = get_covered_boundaries(&sensors, y, left_boundary, right_boundary)
        // .inspect(|r| println!("Coalesced: {}->{} ", r.start(), r.end()))
        .map(|r| -> isize {
            r.end() - r.start() + 1
                - (sensors
                    .iter()
                    .filter(|s| s.position.y == y && r.contains(&s.position.x))
                    .map(|s| s.position.clone())
                    .chain(
                        sensors
                            .iter()
                            .filter(|s| s.nearest_beacon.y == y && r.contains(&s.nearest_beacon.x))
                            .map(|s| s.nearest_beacon.clone()),
                    )
                    .unique()
                    .count()) as isize
        })
        .sum::<isize>();

    Ok(covered_by_beacon)
}

fn get_covered_boundaries(
    sensors: &[Sensor],
    y: isize,
    left_boundary: isize,
    right_boundary: isize,
) -> impl Iterator<Item = RangeInclusive<isize>> {
    sensors
        .iter()
        .flat_map(|s| s.row_boundary(y))
        .filter_map(|r| {
            if r.start() < &left_boundary {
                if r.contains(&left_boundary) {
                    return Some(left_boundary..=*r.end());
                } else {
                    None
                }
            } else if r.end() > &right_boundary {
                if r.contains(&right_boundary) {
                    return Some(*r.start()..=right_boundary);
                } else {
                    None
                }
            } else {
                Some(r)
            }
        })
        .sorted_by_key(|s| *s.start())
        .coalesce(|l, r| {
            if l.contains(r.start()) || l.contains(r.end()) {
                Ok(min(*l.start(), *r.start())..=max(*l.end(), *r.end()))
            } else if l.end() + 1 == *r.start() {
                Ok(*l.start()..=*r.end())
            } else {
                Err((l, r))
            }
        })
}

fn search_for_distress_beacon(input: &str, start: isize, end: isize) -> Result<isize> {
    let sensors = parse_sensors(input)?;

    for y in start..=end {
        if let Some(second_range) = get_covered_boundaries(&sensors, y, start, end).nth(1) {
            let point = Point {
                x: second_range.start() - 1,
                y,
            };
            return Ok(point.x * 4000000 + point.y);
        }
    }
    Err(eyre!("Got nothing"))
}

fn parse_sensors(input: &str) -> Result<Vec<Sensor>> {
    input
        .lines()
        .map(|l| {
            parse_sensor(l)
                .finish()
                .map(|(_, s)| s)
                .map_err(|e| eyre!("Parse error:\n{e}"))
        })
        .collect::<Result<Vec<_>>>()
}
// Sensor at x=9, y=16: closest beacon is at x=10, y=16
fn parse_sensor(line: &str) -> IResult<&str, Sensor> {
    map(
        tuple((
            preceded(tag("Sensor at "), parse_point),
            preceded(tag(": closest beacon is at "), parse_point),
        )),
        |(s, b)| Sensor::new(s, b),
    )(line)
}
fn parse_point(s: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            preceded(tag("x="), nom::character::complete::i64),
            tag(", "),
            preceded(tag("y="), nom::character::complete::i64),
        ),
        |(x, y)| Point {
            x: x as isize,
            y: y as isize,
        },
    )(s)
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn distance_to(&self, other: &Self) -> usize {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}
struct Sensor {
    position: Point,
    nearest_beacon: Point,
    nearest_beacon_distance: usize,
}
impl Sensor {
    fn new(position: Point, nearest_beacon: Point) -> Self {
        Self {
            nearest_beacon_distance: position.distance_to(&nearest_beacon),
            position,
            nearest_beacon,
        }
    }

    fn row_boundary(&self, row: isize) -> Option<RangeInclusive<isize>> {
        let offset = self.position.y.abs_diff(row);
        if offset > self.nearest_beacon_distance {
            return None;
        }
        let distance_adjustment = self.nearest_beacon_distance - offset;
        Some(
            (self.position.x - distance_adjustment as isize)
                ..=(self.position.x + distance_adjustment as isize),
        )
    }
}
impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor(Position:x={}, y={} Beacon:x={}, y={})",
            self.position.x, self.position.y, self.nearest_beacon.x, self.nearest_beacon.y
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{columns_without_beacon_optimized, search_for_distress_beacon};

    #[test]
    fn example_part1() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        // assert_eq!(26, columns_without_beacon(input, 10).unwrap());
        assert_eq!(26, columns_without_beacon_optimized(input, 10).unwrap());
    }

    #[test]
    fn example_part2() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(56000011, search_for_distress_beacon(input, 0, 20).unwrap());
    }
}
