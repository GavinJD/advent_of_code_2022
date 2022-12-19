use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use color_eyre::{
    eyre::{eyre, Result},
    Report,
};
use itertools::Itertools;
fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_12.txt");
    let grid = Grid::from_str(input)?;
    println!("Part 1: {}", find_shortest_path(&grid)?);
    println!("Part 2: {}", find_shortest_start_for_path(&grid)?);
    Ok(())
}

fn find_shortest_path(grid: &Grid) -> Result<u64> {
    grid.distance_from_end
        .get(&grid.start)
        .cloned()
        .ok_or(eyre!("No path from start to end?"))
}

fn find_shortest_start_for_path(grid: &Grid) -> Result<u64> {
    grid.distance_from_end
        .iter()
        .filter(|(pos, _)| grid.points[pos.0][pos.1].0 == 0)
        .min_by_key(|(_, d)| *d)
        .map(|(_, d)| d)
        .cloned()
        .ok_or(eyre!("Nothing in grid!"))
}

struct Grid {
    width: usize,
    height: usize,
    points: Vec<Vec<GridPoint>>,
    start: GridPos,
    end: GridPos,
    distance_from_end: HashMap<GridPos, u64>,
}
impl FromStr for Grid {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .map(|l| l.len())
            .ok_or(eyre!("First row is empty!"))?;
        let height = s.lines().count();
        let mut start = None;
        let mut end = None;
        let points = s
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == 'S' {
                            start = Some((i, j));
                            GridPoint::from_char('a')
                        } else if c == 'E' {
                            end = Some((i, j));
                            GridPoint::from_char('z')
                        } else {
                            GridPoint::from_char(c)
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        Ok(Grid::new(
            width,
            height,
            points,
            start.ok_or(eyre!("Start point not found in grid"))?,
            end.ok_or(eyre!("End point not found in grid"))?,
        ))
    }
}

impl Grid {
    fn in_bound(&self, p: &GridPos) -> bool {
        p.0 < self.height && p.1 < self.width
    }

    fn can_move(&self, start: &GridPos, end: &GridPos) -> bool {
        if !self.in_bound(start) || !self.in_bound(end) {
            return false;
        }

        let end_height = self.points[end.0][end.1].0;
        let start_height = self.points[start.0][start.1].0;

        // end_height <= start_height || start_height + 1 == end_height
        start_height.saturating_sub(1) == end_height || start_height <= end_height
    }

    fn new(
        width: usize,
        height: usize,
        points: Vec<Vec<GridPoint>>,
        start: GridPos,
        end: GridPos,
    ) -> Self {
        let mut r = Self {
            width,
            height,
            points,
            start,
            end,
            distance_from_end: HashMap::new(),
        };
        r.precompute_distance_from_end();
        r
    }

    fn precompute_distance_from_end(&mut self) {
        let mut visited: HashSet<GridPos> = HashSet::new();

        loop {
            // println!("Visited: {visited:?}");
            // println!("Distance: {distance:?}");
            let (current_node, current_node_distance) = if visited.is_empty() {
                (self.end, 0)
            } else if let Some(n) = self
                .distance_from_end
                .iter()
                .filter(|(k, _)| !visited.contains(k))
                .min_by_key(|(_k, v)| **v)
                .map(|(k, v)| (*k, *v))
            {
                n
            } else {
                break;
            };

            let neighbours = [
                (current_node.0.saturating_sub(1), current_node.1),
                (current_node.0 + 1, current_node.1),
                (current_node.0, current_node.1.saturating_sub(1)),
                (current_node.0, current_node.1 + 1),
            ];
            let neighbours = neighbours
                .iter()
                .filter(|neighbour| {
                    !visited.contains(neighbour) && self.can_move(&current_node, neighbour)
                })
                .collect_vec();

            for neighbour in neighbours {
                self.distance_from_end
                    .entry(*neighbour)
                    .and_modify(|d| *d = Ord::min(*d, current_node_distance + 1))
                    .or_insert(current_node_distance + 1);
            }
            visited.insert(current_node);
        }

        // unreachable!()
    }
}

type GridPos = (usize, usize);
struct GridPoint(u8);
impl GridPoint {
    fn from_char(c: char) -> Self {
        GridPoint(c as u8 - b'a')
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.points
                .iter()
                .map(|line| line.iter().map(|i| (i.0 + b'a') as char).join(""))
                .join("\n")
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "{}",
            self.points
                .iter()
                .map(|line| line.iter().map(|i| format!(" {:2} ", i.0)).join(""))
                .join("\n")
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{find_shortest_path, find_shortest_start_for_path, Grid};

    #[test]
    fn example_part1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(
            31,
            find_shortest_path(&Grid::from_str(input).unwrap()).unwrap()
        );
    }

    #[test]
    fn example_part2() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(
            29,
            find_shortest_start_for_path(&Grid::from_str(input).unwrap()).unwrap()
        );
    }
}
