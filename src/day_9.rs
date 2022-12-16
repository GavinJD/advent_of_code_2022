use std::{collections::HashSet, f64::consts::SQRT_2};

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_9.txt");
    println!("Part 1: {}", unique_tail_positions(input, 2)?);
    println!("Part 2: {}", unique_tail_positions(input, 10)?);
    Ok(())
}

fn unique_tail_positions(input: &str, rope_size: usize) -> Result<usize> {
    let movements = parse_movements(input)?;

    let mut rope = Rope {
        knots: (0..rope_size).map(|_| (0, 0).into()).collect_vec(),
    };
    // dbg!(&rope);
    // visualize(&rope);

    let mut tail_positions: HashSet<MapPos> = HashSet::new();
    tail_positions.insert(rope.knots[rope_size - 1]);

    for movement in movements {
        for _ in 0..movement.count {
            move_rope(&mut rope, movement.direction)?;
            tail_positions.insert(rope.knots[rope_size - 1]);
            // println!("Move {:?}", movement.direction);
            // visualize(&rope);
        }
        // println!("Move {:?} {} times", movement.direction, movement.count);
        // visualize(&rope);
    }

    Ok(tail_positions.len())
}

fn visualize(rope: &Rope) {
    for p in (-5..5).rev() {
        for q in -5..5 {
            let current: MapPos = (q, p).into();
            let char = if let Some((pos, _)) = rope.knots.iter().find_position(|r| **r == current) {
                if pos == 0 {
                    "H".to_owned()
                } else if pos == rope.knots.len() - 1 {
                    "T".to_owned()
                } else {
                    pos.to_string()
                }
            } else {
                ".".to_owned()
            };
            print!("{char}");
        }
        println!();
    }
    println!(
        "Rope head: {:?}, tail: {:?}",
        rope.knots.first().unwrap(),
        rope.knots.last().unwrap()
    );
}

fn parse_movements(input: &str) -> Result<Vec<Movement>> {
    input
        .lines()
        .map(|l| -> Result<Movement> {
            let (direction, count) = l.split_once(' ').ok_or(eyre!("Unable to parse line"))?;
            let direction = match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => return Err(eyre!("Unrecognized direction")),
            };
            let count = count
                .parse::<u64>()
                .map_err(|_| eyre!("Could not parse number of directions"))?;

            Ok(Movement { direction, count })
        })
        .collect::<Result<Vec<_>>>()
}

fn move_rope(rope: &mut Rope, direction: Direction) -> Result<()> {
    let first_knot = rope.knots.first_mut().ok_or(eyre!("Rope is empty!"))?;
    match direction {
        Direction::Up => first_knot.y += 1,
        Direction::Down => first_knot.y -= 1,
        Direction::Left => first_knot.x -= 1,
        Direction::Right => first_knot.x += 1,
    }

    for i in 0..(rope.knots.len() - 1) {
        let head = rope.knots.get(i).cloned().unwrap();
        let tail = rope.knots.get_mut(i + 1).unwrap();
        let distance = head.distance_from(tail);
        if distance > SQRT_2 {
            let dist_x = (head.x - tail.x) as f32;
            let dist_y = (head.y - tail.y) as f32;

            tail.x += round_to_digit(dist_x / 2.0) as i64;
            tail.y += round_to_digit(dist_y / 2.0) as i64;
        }
    }

    Ok(())
}

fn round_to_digit(x: f32) -> f32 {
    if x.is_sign_positive() {
        x.ceil()
    } else {
        x.floor()
    }
}

struct Movement {
    direction: Direction,
    count: u64,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<MapPos>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct MapPos {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for MapPos {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl MapPos {
    fn distance_from(&self, other: &MapPos) -> f64 {
        f64::sqrt(((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::unique_tail_positions;

    #[test]
    fn example_part1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, unique_tail_positions(input, 2).unwrap());
    }

    #[test]
    fn example_part2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, unique_tail_positions(input, 10).unwrap());
    }
}
