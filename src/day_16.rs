use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use itertools::Itertools;
use nom::bytes::complete::{tag, take};
use nom::{branch::alt, error::Error, multi::separated_list0, sequence::tuple};
use petgraph::algo::floyd_warshall;
use petgraph::prelude::*;

use color_eyre::eyre::{eyre, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_16.txt");
    println!("Part 1: {}", get_max_possible_pressure_released(input)?);
    println!(
        "Part 2: {}",
        get_max_possible_pressure_with_elephant(input)?
    );
    Ok(())
}

const MAX_TIME: u64 = 30;

fn get_max_possible_pressure_released(input: &str) -> Result<u64> {
    let tunnels = parse_tunnels(input)?;

    let current_time = 0;
    let total_released_pressure = 0;
    let current_tunnel = tunnels.nodes().find(|t| t.valve == ['A', 'A']).unwrap();

    let cost_matrix = floyd_warshall(&tunnels, |edge| {
        if tunnels.contains_edge(edge.source(), edge.target()) {
            1
        } else {
            u64::MAX
        }
    })
    .map_err(|e| eyre!("Negative cycle found! {e:?}"))?;

    Ok(tunnels
        .nodes()
        .filter(|t| t.flow_rate > 0)
        .map(|node| {
            find_max_releasable_pressure_if_moved(
                &tunnels,
                &cost_matrix,
                &current_tunnel,
                &node,
                HashSet::new(),
                current_time,
                total_released_pressure,
            )
        })
        .max()
        .unwrap_or(0))
}

fn get_max_possible_pressure_with_elephant(input: &str) -> Result<u64> {
    let tunnels = parse_tunnels(input)?;

    let current_time = 4;
    let total_released_pressure = 0;
    let current_tunnel = tunnels.nodes().find(|t| t.valve == ['A', 'A']).unwrap();
    let to_open_tunnels = tunnels.nodes().filter(|t| t.flow_rate > 0).collect_vec();

    let cost_matrix = floyd_warshall(&tunnels, |edge| {
        if tunnels.contains_edge(edge.source(), edge.target()) {
            1
        } else {
            u64::MAX
        }
    })
    .map_err(|e| eyre!("Negative cycle found! {e:?}"))?;

    Ok(all_combinations_split_in_2(&to_open_tunnels)
        .filter(|(s1, s2)| s1.len() > (to_open_tunnels.len() / 2 - 1) && s2.len() > (to_open_tunnels.len() / 2 - 1))
        .enumerate()
        .map(|(_i, (s1, s2))| {
            // if _i % 1000 == 0 {
            //     println!("  At {_i}...");
            // }
            tunnels
                .nodes()
                .filter(|t| t.flow_rate > 0)
                .filter(|t| !s1.contains(t))
                .map(|node| {
                    find_max_releasable_pressure_if_moved(
                        &tunnels,
                        &cost_matrix,
                        &current_tunnel,
                        &node,
                        s1.clone(),
                        current_time,
                        total_released_pressure,
                    )
                })
                .max()
                .unwrap_or(0)
                + tunnels
                    .nodes()
                    .filter(|t| t.flow_rate > 0)
                    .filter(|t| !s2.contains(t))
                    .map(|node| {
                        find_max_releasable_pressure_if_moved(
                            &tunnels,
                            &cost_matrix,
                            &current_tunnel,
                            &node,
                            s2.clone(),
                            current_time,
                            total_released_pressure,
                        )
                    })
                    .max()
                    .unwrap_or(0)
        })
        .max()
        .unwrap_or(0))
}

fn find_max_releasable_pressure_if_moved(
    tunnel_map: &DiGraphMap<Tunnel, u64>,
    cost_matrix: &HashMap<(Tunnel, Tunnel), u64>,
    previous_tunnel: &Tunnel,
    new_tunnel: &Tunnel,
    mut opened_tunnels: HashSet<Tunnel>,
    previous_time: u64,
    previously_released_pressure: u64,
) -> u64 {
    if previous_time > MAX_TIME {
        return previously_released_pressure;
    }

    let distance_from_previous_to_new = *cost_matrix
        .get(&(*previous_tunnel, *new_tunnel))
        .unwrap_or(&MAX_TIME);
    let new_time = previous_time + distance_from_previous_to_new + 1;

    if new_time > MAX_TIME {
        return previously_released_pressure;
    }

    let current_pressure_released = previously_released_pressure
        + pressure_if_moved_and_opened(
            new_tunnel.flow_rate,
            distance_from_previous_to_new,
            previous_time,
            opened_tunnels.contains(new_tunnel),
        );
    opened_tunnels.insert(*new_tunnel);

    if new_time == MAX_TIME
        || opened_tunnels.len() == tunnel_map.nodes().filter(|n| n.flow_rate > 0).count()
    {
        return current_pressure_released;
    }

    tunnel_map
        .nodes()
        .filter(|t| !opened_tunnels.contains(t))
        .filter(|t| t.flow_rate > 0)
        .map(|neighbor| {
            find_max_releasable_pressure_if_moved(
                tunnel_map,
                cost_matrix,
                new_tunnel,
                &neighbor,
                opened_tunnels.clone(),
                new_time,
                current_pressure_released,
            )
        })
        .max()
        .unwrap_or(current_pressure_released)
}

fn pressure_if_moved_and_opened(
    destination_tunnel_flow_rate: u64,
    length_to_tunnel: u64,
    current_time: u64,
    is_destination_open: bool,
) -> u64 {
    if is_destination_open {
        0
    } else {
        let time_valve_is_open = MAX_TIME.saturating_sub(
            current_time
                .saturating_add(length_to_tunnel)
                .saturating_add(1),
        );
        time_valve_is_open * destination_tunnel_flow_rate
    }
}

fn parse_tunnels(s: &str) -> Result<DiGraphMap<Tunnel, u64>> {
    let mut result = DiGraphMap::new();
    let mut paths: Vec<(Tunnel, [char; 2])> = Vec::new();
    for line in s.lines() {
        let (valve, rate, tunnel_paths) = parse_tunnel_line_description(line)?;
        let tunnel = Tunnel::new(valve, rate);
        result.add_node(tunnel);

        for path in tunnel_paths.into_iter().map(|p| (tunnel, p)) {
            paths.push(path);
        }
    }

    for (source_tunnel, end_tunnel_valve) in paths {
        result.add_edge(
            source_tunnel,
            result
                .nodes()
                .find(|t| t.valve == end_tunnel_valve)
                .ok_or_else(|| eyre!("Cannot find {}", end_tunnel_valve.iter().join("")))?,
            1,
        );
    }

    Ok(result)
}
fn parse_tunnel_line_description(l: &str) -> Result<([char; 2], u64, std::vec::Vec<[char; 2]>)> {
    tuple((
        tag::<_, _, Error<&str>>("Valve "),
        take(2usize),
        tag(" has flow rate="),
        nom::character::complete::u64,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list0(tag(", "), take(2usize)),
    ))(l)
    .map(|(_, o)| o)
    .map(|(_, valve, _, rate, _, paths)| {
        (
            [valve.chars().next().unwrap(), valve.chars().nth(1).unwrap()],
            rate,
            paths
                .into_iter()
                .map(|path| [path.chars().next().unwrap(), path.chars().nth(1).unwrap()])
                .collect_vec(),
        )
    })
    .map_err(|e| eyre!("Parsing Error::{}\n\n", e))
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Tunnel {
    valve: [char; 2],
    flow_rate: u64,
}

impl Tunnel {
    fn new(valve: [char; 2], flow_rate: u64) -> Self {
        Self { valve, flow_rate }
    }
}
impl std::fmt::Debug for Tunnel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            // "Tunnel(valve={}, flow_rate={}, {}) ",
            self.valve.iter().join(""),
            // self.flow_rate,
            // if self.open { "open" } else { "not open" }
        )?;
        Ok(())
    }
}

fn all_combinations_split_in_2<T: Copy + Eq + Hash>(
    to_split: &[T],
) -> impl Iterator<Item = (HashSet<T>, HashSet<T>)> + '_ {
    // println!("Going though {} combinations...", 2usize.pow(to_split.len() as u32));
    (0..2usize.pow(to_split.len() as u32)).map(|current| {
        let mut set1: HashSet<T> = HashSet::with_capacity(to_split.len());
        let mut set2: HashSet<T> = HashSet::with_capacity(to_split.len());

        for (bit, item) in to_split.iter().enumerate() {
            if (current & (1 << bit)) > 0 {
                set1.insert(*item);
            } else {
                set2.insert(*item);
            }
        }

        (set1, set2)
    })
}

#[cfg(test)]
mod tests {
    use crate::{get_max_possible_pressure_released, get_max_possible_pressure_with_elephant};

    #[test]
    fn example_part1() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(1651, get_max_possible_pressure_released(input).unwrap());
    }

    #[test]
    fn example_part2() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(
            1707,
            get_max_possible_pressure_with_elephant(input).unwrap()
        );
    }
}
