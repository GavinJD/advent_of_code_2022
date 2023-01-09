use std::ops::Add;

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_18.txt");

    println!("Part 1: {}", surface_area_of_lava_droplets(input, true)?);
    println!("Part 2: {}", surface_area_of_lava_droplets(input, false)?);
    Ok(())
}

fn surface_area_of_lava_droplets(input: &str, with_interior: bool) -> Result<i64> {
    let droplets: Vec<Point> = parse_points(input)?;

    let directions = [
        Point::new(1, 0, 0),
        Point::new(-1, 0, 0),
        Point::new(0, 1, 0),
        Point::new(0, -1, 0),
        Point::new(0, 0, 1),
        Point::new(0, 0, -1),
    ];

    let mut outer_air = Vec::new();

    if !with_interior {
        let max = 22;

        let init = Point::new(0, 0, 0);
        outer_air.push(init);
        let mut idx = 0;
        while idx < outer_air.len() {
            for dir in directions.iter() {
                let point = outer_air[idx] + *dir;
                if point.x > max
                    || point.x < -1
                    || point.y > max
                    || point.y < -1
                    || point.z > max
                    || point.z < -1
                {
                    continue;
                }
                if !droplets.contains(&point) && !outer_air.contains(&point) {
                    outer_air.push(point);
                }
            }
            idx += 1;
        }
    }

    Ok(droplets
        .iter()
        .map(|droplet| {
            directions
                .iter()
                .map(|dir| {
                    let point = *droplet + *dir;
                    if droplets.contains(&point) {
                        0
                    } else if with_interior || outer_air.contains(&point) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i64>()
        })
        .sum())
}

fn parse_points(input: &str) -> Result<Vec<Point>, color_eyre::Report> {
    input
        .lines()
        .map(|line| {
            let d: (_, _, _) = line
                .split(',')
                .map(|s| s.parse::<i64>())
                .collect_tuple()
                .ok_or(eyre!("Line does not have 3 numbers"))?;
            Ok(Point::new(d.0?, d.1?, d.2?))
        })
        .collect::<Result<Vec<_>>>()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}
impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Point { x, y, z }
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)?;
        Ok(())
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::surface_area_of_lava_droplets;

    #[test]
    fn example_part1() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(64, surface_area_of_lava_droplets(input, true).unwrap());
    }

    #[test]
    fn example_part2() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(58, surface_area_of_lava_droplets(input, false).unwrap());
    }
}
