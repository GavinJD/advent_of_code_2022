use std::collections::HashSet;

use color_eyre::eyre::{eyre, Result};
use itertools::{Either, Itertools};
use nom::{
    bytes::complete::tag, combinator::map, error::Error, multi::separated_list0, sequence::tuple,
    Finish,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_14.txt");
    println!(
        "Part 1: {}",
        fill_with_sand(input, FillWithSandUntil::RockBoundary)?
    );
    println!(
        "Part 2: {}",
        fill_with_sand(input, FillWithSandUntil::Floor)?
    );
    Ok(())
}

fn fill_with_sand(input: &str, until: FillWithSandUntil) -> Result<usize> {
    let mut cave = Cave::new(input)?;
    // println!("{cave}");

    loop {
        // if i % 10 == 0 {
        //     println!("{cave}");
        // }
        let mut sand_position = cave.sand_source;
        let mut sand_path = Vec::new();
        let mut falling = true;

        while falling {
            match until {
                FillWithSandUntil::RockBoundary => {
                    if sand_position.1 > cave.bottom_boundary {
                        break;
                    }
                }
                FillWithSandUntil::Floor => {
                    if sand_position.1 + 1 == cave.bottom_boundary + FLOOR_OFFSET {
                        // reached floor
                        falling = false;
                        break;
                    }
                }
            }
            sand_path.push(sand_position);
            if !cave.occupied(&(sand_position.0, sand_position.1 + 1)) {
                sand_position = (sand_position.0, sand_position.1 + 1);
            } else if !cave.occupied(&(sand_position.0 - 1, sand_position.1 + 1)) {
                sand_position = (sand_position.0 - 1, sand_position.1 + 1);
            } else if !cave.occupied(&(sand_position.0 + 1, sand_position.1 + 1)) {
                sand_position = (sand_position.0 + 1, sand_position.1 + 1);
            } else {
                falling = false;
            }
        }

        let should_sand_not_be_added = match until {
            FillWithSandUntil::RockBoundary => falling, // if sand is falling still, means we've
            // gone past rock boundary
            FillWithSandUntil::Floor => cave.sand.contains(&cave.sand_source),
        };

        if should_sand_not_be_added {
            cave.terminal_sand_path = sand_path;
            break;
        } else {
            cave.sand.insert(sand_position);
        }
    }
    // println!("\n\n{cave}");

    Ok(cave.sand.len())
}

const FLOOR_OFFSET: usize = 2;
type CavePos = (usize, usize);
struct Cave {
    sand_source: CavePos,
    rocks: HashSet<CavePos>,
    sand: HashSet<CavePos>,
    bottom_boundary: usize,
    terminal_sand_path: Vec<CavePos>,
}
enum FillWithSandUntil {
    RockBoundary,
    Floor,
}

impl Cave {
    fn new(rock_paths: &str) -> Result<Self> {
        let rocks = parse_rocks(rock_paths)?;
        let bottom_boundary = rocks.iter().map(|(_, y)| y).max().cloned().unwrap_or(0);
        Ok(Self {
            sand_source: (500, 0),
            rocks,
            sand: HashSet::new(),
            bottom_boundary,
            terminal_sand_path: Vec::new(),
        })
    }

    fn occupied(&self, pos: &CavePos) -> bool {
        self.sand.contains(pos) || self.rocks.contains(pos)
    }
}

fn parse_rocks(input: &str) -> Result<HashSet<CavePos>> {
    input
        .lines()
        .map(parse_rock_paths)
        .map(|paths| {
            paths.map(|p| {
                p.into_iter().tuple_windows().flat_map(|(pos1, pos2)| {
                    if pos1.0 < pos2.0 {
                        Either::Left(pos1.0..=pos2.0)
                    } else {
                        Either::Right((pos2.0..=pos1.0).rev())
                    }
                    .into_iter()
                    .flat_map(move |x| {
                        if pos1.1 < pos2.1 {
                            Either::Left(pos1.1..=pos2.1)
                        } else {
                            Either::Right((pos2.1..=pos1.1).rev())
                        }
                        .into_iter()
                        .map(move |y| (x, y))
                    })
                })
            })
        })
        .flatten_ok()
        .collect::<Result<HashSet<_>>>()
}

fn parse_rock_paths(input: &str) -> Result<Vec<CavePos>> {
    separated_list0(
        tag::<_, _, Error<&str>>(" -> "),
        map(
            tuple((
                nom::character::complete::u64,
                tag(","),
                nom::character::complete::u64,
            )),
            |(a, _, b)| (a as usize, b as usize),
        ),
    )(input)
    .finish()
    .map(|(_, paths)| paths)
    .map_err(|e| eyre!("Parse Error:\n{e}\n\n"))
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let (left_boundary, right_boundary) = match self.rocks.iter().map(|(x, _)| x).minmax() {
        //     MinMaxResult::NoElements => (0, 0),
        //     MinMaxResult::OneElement(i) => (*i, *i),
        //     MinMaxResult::MinMax(i, j) => (*i, *j),
        // };
        let (left_boundary, right_boundary) =
            (500 - self.bottom_boundary, 500 + self.bottom_boundary);

        for y in 0..=(self.bottom_boundary + 1) {
            for x in left_boundary..=right_boundary {
                let c = if self.sand_source == (x, y) {
                    "+"
                } else if self.rocks.contains(&(x, y)) {
                    "#"
                } else if self.sand.contains(&(x, y)) {
                    "o"
                } else if self.terminal_sand_path.contains(&(x, y)) {
                    "~"
                } else {
                    "."
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{fill_with_sand, FillWithSandUntil};

    #[test]
    fn example_part1() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(
            24,
            fill_with_sand(input, FillWithSandUntil::RockBoundary).unwrap()
        );
    }
    #[test]
    fn example_part2() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(93, fill_with_sand(input, FillWithSandUntil::Floor).unwrap());
    }
}
