use std::{cmp::min, fmt::Display, time::Instant};

use color_eyre::eyre::{eyre, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_17.txt");
    let directions = parse_directions(input)?;
    println!(
        "Part 1: {}",
        simulate_and_get_highest_rock(2022, &directions)
    );
    println!(
        "Part 2: {}",
        simulate_and_get_highest_rock(1_000_000_000_000, &directions)
    );
    Ok(())
}

fn cycle_check(deltas: &[usize], minimum_length: usize) -> Option<&[usize]> {
    if deltas.len() < minimum_length {
        return None;
    }

    for i in ((deltas.len() / 2)..deltas.len().saturating_sub(3)).rev() {
        let mut length = 1;
        let max_possible_length = deltas.len() - i;

        if max_possible_length < minimum_length {
            continue;
        }

        while deltas.get(i - length) == deltas.get(deltas.len() - length)
            && length < max_possible_length
        {
            length += 1;
        }

        if length == max_possible_length {
            // confirm that this is a cycle by going back one more time
            let c1 = deltas.get((i - length)..(i));
            let c2 = deltas.get(i..deltas.len());

            if c1.is_some()
                && c2.is_some()
                && c1
                    .unwrap()
                    .iter()
                    .zip(c2.unwrap().iter())
                    .all(|(a, b)| a == b)
            {
                return deltas.get((i)..(deltas.len()));
            }
        }
    }
    None
}

fn parse_directions(input: &str) -> Result<Vec<Direction>> {
    input
        .trim()
        .chars()
        .into_iter()
        .map(|c| match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(eyre!("Unrecognized character :: '{}'", c)),
        })
        .collect::<Result<Vec<_>>>()
}

fn simulate_and_get_highest_rock(steps: usize, directions: &[Direction]) -> usize {
    simulate(steps, directions).highest_empty_row_index
}

fn simulate(number_of_rocks: usize, directions: &[Direction]) -> Arena {
    let mut direction_counter = 0;

    let mut arena = Arena::new();
    let mut current_shape;

    let mut highest = 0;
    let mut deltas = Vec::new();
    let start = Instant::now();

    for shape in 0..number_of_rocks {
        current_shape = Shape::make_shape(shape % Shape::MAX_SHAPES);

        let mut x_pos = 2usize;
        let mut remaining_till_bottom = 3;
        while remaining_till_bottom > 0 {
            let current_dir = directions[direction_counter];
            // println!("Moving {current_dir:?}");
            match current_dir {
                Direction::Left => x_pos = x_pos.saturating_sub(1),
                Direction::Right => x_pos = min(x_pos + 1, arena.width - current_shape.width),
                _ => unreachable!(),
            }
            direction_counter = (direction_counter + 1) % directions.len();
            remaining_till_bottom -= 1
        }

        current_shape.lower_left_corner = (x_pos, arena.get_first_empty_row());
        // arena.draw_shape(&current_shape);
        // println!("{arena}");

        loop {
            let current_direction = directions[direction_counter];
            let _dir_movement = arena.move_shape(&mut current_shape, current_direction);
            // println!(
            //     "Moving {:?} :: {}",
            //     current_direction,
            //     if _dir_movement { "success" } else { "fail" }
            // );
            direction_counter = (direction_counter + 1) % directions.len();

            let down_movement = arena.move_shape(&mut current_shape, Direction::Down);
            // println!(
            //     "Moving Down :: {}",
            //     if down_movement { "success" } else { "fail" }
            // );
            if !down_movement {
                arena.draw_shape(&current_shape);
                deltas.push(arena.highest_empty_row_index - highest);
                highest = arena.highest_empty_row_index;

                if let Some(cycle) = cycle_check(&deltas, directions.len()) {
                    //  println!("Cycle found at {shape}");
                    //  println!(
                    //      "Last {} elements:\n{}\n",
                    //      cycle.len(),
                    //      deltas.iter().skip(deltas.len() - cycle.len()).join(",")
                    //  );
                    let remaining_rocks = number_of_rocks - shape - 1;

                    let cycle_sum: usize = cycle.iter().sum();
                    let cycle_hops = remaining_rocks / cycle.len();
                    highest += cycle_hops * cycle_sum;
                    let remaining_rocks = remaining_rocks % cycle.len();

                    for i in 0..remaining_rocks {
                        highest += cycle[i % cycle.len()];
                    }
                    arena.highest_empty_row_index = highest;
                    return arena;
                }

                // println!("Rock {shape}\n{arena}");
                // if shape % 1000 == 0 {
                //     println!("Reached {shape}");
                // }
                if shape == 1_000_000 {
                    let elapsed = start.elapsed();
                    println!("Time for 10 million rocks :: {}ms", elapsed.as_millis());
                    dbg!(&arena.points.len());
                    println!(
                        "Time to complete :: {}s",
                        (elapsed * (number_of_rocks / 1_000_000) as u32).as_secs()
                    );
                }
                break;
            }
        }
    }

    arena
}

type Point = (usize, usize);

struct Arena {
    width: usize,
    points: Vec<Cell>,
    highest_empty_row_index: usize,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Rock,
    Air,
}
#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Down,
}
struct Shape {
    lower_left_corner: Point,
    width: usize,
    height: usize,
    points: Vec<Cell>,
}

impl Shape {
    const MAX_SHAPES: usize = 5;
    fn make_shape(idx: usize) -> Self {
        match idx {
            0 => Shape {
                lower_left_corner: (0, 0),
                width: 4,
                height: 1,
                points: vec![Cell::Rock; 4],
            },
            1 => Shape {
                lower_left_corner: (0, 0),
                width: 3,
                height: 3,
                points: vec![
                    Cell::Air,
                    Cell::Rock,
                    Cell::Air,
                    Cell::Rock,
                    Cell::Rock,
                    Cell::Rock,
                    Cell::Air,
                    Cell::Rock,
                    Cell::Air,
                ],
            },
            2 => Shape {
                lower_left_corner: (0, 0),
                width: 3,
                height: 3,
                points: vec![
                    Cell::Rock,
                    Cell::Rock,
                    Cell::Rock,
                    Cell::Air,
                    Cell::Air,
                    Cell::Rock,
                    Cell::Air,
                    Cell::Air,
                    Cell::Rock,
                ],
            },
            3 => Shape {
                lower_left_corner: (0, 0),
                width: 1,
                height: 4,
                points: vec![Cell::Rock; 4],
            },
            4 => Shape {
                lower_left_corner: (0, 0),
                width: 2,
                height: 2,
                points: vec![Cell::Rock; 4],
            },
            _ => unreachable!(),
        }
    }

    fn get_point(&self, point: Point) -> Cell {
        self.points[point.0 as usize + point.1 as usize * self.width]
    }
}
impl Arena {
    fn new() -> Self {
        Self {
            width: 7,
            points: vec![Cell::Air; 11_000_000],
            highest_empty_row_index: 0,
        }
    }
    fn get_point(&self, point: Point) -> Cell {
        if point.0 >= self.width {
            Cell::Rock
        } else if point.1 >= self.highest_empty_row_index {
            Cell::Air
        } else {
            self.points[point.0 + point.1 * self.width]
        }
    }

    fn set_point(&mut self, point: Point, c: Cell) {
        self.points[point.0 + point.1 * self.width] = c;
    }
    fn row_count(&self) -> usize {
        self.highest_empty_row_index
    }

    fn draw_shape(&mut self, s: &Shape) {
        let required_arena_height = s.lower_left_corner.1 + s.height;
        self.extend_height_with_air(required_arena_height);

        // debug_assert_eq!(Cell::Air, self.get_point(s.lower_left_corner));

        for y in 0..s.height {
            for x in 0..s.width {
                // println!(
                //     "{:?} -> ({}, {})",
                //     s.get_point((x, y)),
                //     x + s.lower_left_corner.0,
                //     y + s.lower_left_corner.1
                // );
                let shape_cell = s.get_point((x, y));

                // no need to draw air
                if shape_cell == Cell::Rock {
                    let point_to_draw_to = (x + s.lower_left_corner.0, y + s.lower_left_corner.1);
                    debug_assert_eq!(Cell::Air, self.get_point(point_to_draw_to));
                    self.set_point(point_to_draw_to, shape_cell);
                }
            }
        }
    }

    // fn clear_shape(&mut self, s: &Shape) {
    //     for y in 0..s.height {
    //         for x in 0..s.width {
    //             let shape_cell = s.get_point((x, y));
    //             if shape_cell == Cell::Rock {
    //                 let point_to_clear = (x + s.lower_left_corner.0, y + s.lower_left_corner.1);
    //                 debug_assert_eq!(Cell::Rock, self.get_point(point_to_clear));
    //                 self.set_point(point_to_clear, Cell::Air);
    //             }
    //         }
    //     }
    // }

    /// return whether movement was successful or not
    fn move_shape(&mut self, s: &mut Shape, dir: Direction) -> bool {
        match dir {
            Direction::Left => {
                if (0..s.height)
                    .map(|y| {
                        (
                            (0..s.width)
                                .find(|x_shape| s.get_point((*x_shape, y)) == Cell::Rock)
                                .unwrap(),
                            y,
                        )
                    })
                    .map(|(x, y)| (x + s.lower_left_corner.0, y + s.lower_left_corner.1))
                    .all(|(x, y)| x != 0 && self.get_point((x - 1, y)) == Cell::Air)
                {
                    // self.clear_shape(s);
                    s.lower_left_corner = (s.lower_left_corner.0 - 1, s.lower_left_corner.1);
                    // self.draw_shape(s);

                    true
                } else {
                    false
                }
            }
            Direction::Right => {
                if (0..s.height)
                    .map(|y| {
                        (
                            (0..s.width)
                                .rev()
                                .find(|x_shape| s.get_point((*x_shape, y)) == Cell::Rock)
                                .unwrap_or(s.width),
                            y,
                        )
                    })
                    .map(|(x, y)| (x + s.lower_left_corner.0, y + s.lower_left_corner.1))
                    .all(|(x, y)| x != self.width - 1 && self.get_point((x + 1, y)) == Cell::Air)
                {
                    // self.clear_shape(s);
                    s.lower_left_corner = (s.lower_left_corner.0 + 1, s.lower_left_corner.1);
                    // self.draw_shape(s);

                    true
                } else {
                    false
                }
            }
            Direction::Down => {
                if (0..s.width)
                    .map(|x_shape| {
                        (
                            x_shape,
                            (0..s.height)
                                .find(|y_shape| s.get_point((x_shape, *y_shape)) == Cell::Rock)
                                .unwrap(),
                        )
                    })
                    .map(|(x, y)| (x + s.lower_left_corner.0, y + s.lower_left_corner.1))
                    .all(|(x, y)| y != 0 && self.get_point((x, y - 1)) == Cell::Air)
                {
                    // self.clear_shape(s);
                    s.lower_left_corner = (s.lower_left_corner.0, s.lower_left_corner.1 - 1);
                    // self.draw_shape(s);

                    true
                } else {
                    false
                }
            }
        }
    }

    fn extend_height_with_air(&mut self, height: usize) {
        if self.highest_empty_row_index < height {
            (0..((height * self.width).saturating_sub(self.points.len())))
                .for_each(|_| self.points.push(Cell::Air));
            self.highest_empty_row_index = height;
        }
    }

    fn get_first_empty_row(&self) -> usize {
        self.highest_empty_row_index
    }
}

impl Display for Arena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..=self.row_count()).rev() {
            write!(f, "|")?;
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.get_point((x, y)) {
                        Cell::Air => ".",
                        Cell::Rock => "#",
                    }
                )?;
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        writeln!(f, "+-------+")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{cycle_check, parse_directions, simulate_and_get_highest_rock};

    #[test]
    fn example_part1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let directions = parse_directions(input).unwrap();
        assert_eq!(3068, simulate_and_get_highest_rock(2022, &directions));
    }

    #[ignore = "not optimized yet"]
    #[test]
    fn example_part2() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let directions = parse_directions(input).unwrap();
        assert_eq!(
            1514285714288,
            simulate_and_get_highest_rock(1_000_000_000_000, &directions)
        );
    }

    #[test]
    fn cycle_check_test() {
        let result = cycle_check(
            &[1, 3, 2, 4, 9, 8, 7, 6, 5, 9, 8, 7, 6, 5, 9, 8, 7, 6, 5],
            4,
        );
        assert!(result.is_some());
        println!("{}", result.unwrap().iter().join(","));
    }
}
