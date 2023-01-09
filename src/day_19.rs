use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_19.txt");

    println!("Part 1: {}", sum_of_quality_levels(input)?);
    println!("Part 2: {}", product_of_max(input, 3)?);
    Ok(())
}

fn sum_of_quality_levels(input: &str) -> Result<u64> {
    let blueprints = input
        .lines()
        .map(parse_blueprint)
        .collect::<Result<Vec<_>>>()?;

    Ok(blueprints
        .into_iter()
        .enumerate()
        .map(|(i, blueprint)| quality_level(i + 1, &blueprint))
        .sum())
}

fn max_geodes_from_blueprint(blueprint: &Blueprint, time: u32) -> u32 {
    // let mut cache = HashMap::new();
    // max_possible_geodes(
    //     &mut cache,
    //     blueprint,
    //     24,
    //     Robots::default(),
    //     Store::default(),
    // )

    Robots::default()
        .possible_robots_possible()
        .into_iter()
        .map(|robot| {
            max_possible_geodes_optimized(
                blueprint,
                time,
                Robots::default(),
                Store::default(),
                robot,
            )
        })
        .max()
        .unwrap_or(0)
}

fn max_possible_geodes_optimized(
    blueprint: &Blueprint,
    mut current_time: u32,
    mut current_bots: Robots,
    mut current_store: Store,
    robot_to_build: RobotType,
) -> u32 {
    while current_time > 0 {
        current_time -= 1;

        let possible_robots = current_store.possible_robots(blueprint);
        current_store.advance(&current_bots);

        if possible_robots.contains(&robot_to_build) {
            current_store.create(robot_to_build, blueprint);
            current_bots.increase(robot_to_build);
            break;
        }
    }

    if current_time == 0 {
        current_store.geode
    } else {
        current_bots
            .possible_robots_possible()
            .into_iter()
            .filter(|robot| match robot {
                RobotType::Ore => {
                    let max_possible_ore = current_bots.ore * current_time + current_store.ore;
                    max_possible_ore < blueprint.ore * current_time
                        || max_possible_ore < blueprint.clay * current_time
                        || max_possible_ore < blueprint.obsidian.0 * current_time
                        || max_possible_ore < blueprint.geode.0 * current_time
                }
                RobotType::Clay => {
                    current_bots.clay * current_time + current_store.clay
                        < blueprint.obsidian.1 * current_time
                }
                RobotType::Obsidian => {
                    current_bots.obsidian * current_time + current_store.obsidian
                        < blueprint.geode.1 * current_time
                }
                RobotType::Geode => true,
            })
            .map(|robot| {
                max_possible_geodes_optimized(
                    blueprint,
                    current_time,
                    current_bots.clone(),
                    current_store.clone(),
                    robot,
                )
            })
            .max()
            .unwrap_or(0)
    }
}

fn max_possible_geodes(
    cache: &mut HashMap<(u32, Robots, Store), u32>,
    blueprint: &Blueprint,
    mut current_time: u32,
    mut current_bots: Robots,
    mut current_store: Store,
) -> (u32, u64) {
    if current_time == 0 {
        return (current_store.geode, 1);
    }
    // if let Some(cached_val) =
    //     cache.get(&(current_time, current_bots.clone(), current_store.clone()))
    // {
    //     // println!("Cache hit! {current_time}s with {current_bots:?} and {current_store:?}");
    //     return (*cached_val, 1);
    // }

    // println!("Checking max_possible_geodes at {current_time}s with {current_bots:?} and {current_store:?}");
    while current_time > 0 {
        // println!("Advancing from {current_time}s");
        current_time -= 1;

        let possible_robots = current_store.possible_robots(blueprint);
        current_store.advance(&current_bots);

        if possible_robots.is_empty() || current_time == 0 {
            continue;
        // } else if possible_robots.contains(&RobotType::Geode) {
        //     current_bots.geode += 1;
        //     current_store.ore -= blueprint.geode.0;
        //     current_store.obsidian -= blueprint.geode.1;
        // } else if current_bots.clay == 0 && possible_robots.contains(&RobotType::Clay) {
        //     // construct clay
        //     current_bots.clay += 1;
        //     current_store.ore -= blueprint.clay;
        //     continue;
        // } else if current_bots.obsidian == 0 && possible_robots.contains(&RobotType::Obsidian) {
        //     //construct obsidian
        //     current_bots.obsidian += 1;
        //     current_store.ore -= blueprint.obsidian.0;
        //     current_store.clay -= blueprint.obsidian.1;
        //     continue;
        } else {
            let mut result = possible_robots
                .into_iter()
                .map(|robot| match robot {
                    RobotType::Ore => max_possible_geodes(
                        cache,
                        blueprint,
                        current_time,
                        current_bots.new_with_increased(1, 0, 0, 0),
                        current_store.new_with_reduced(blueprint.ore, 0, 0, 0),
                    ),
                    RobotType::Clay => max_possible_geodes(
                        cache,
                        blueprint,
                        current_time,
                        current_bots.new_with_increased(0, 1, 0, 0),
                        current_store.new_with_reduced(blueprint.clay, 0, 0, 0),
                    ),
                    RobotType::Obsidian => max_possible_geodes(
                        cache,
                        blueprint,
                        current_time,
                        current_bots.new_with_increased(0, 0, 1, 0),
                        current_store.new_with_reduced(
                            blueprint.obsidian.0,
                            blueprint.obsidian.1,
                            0,
                            0,
                        ),
                    ),
                    RobotType::Geode => max_possible_geodes(
                        cache,
                        blueprint,
                        current_time,
                        current_bots.new_with_increased(0, 0, 0, 1),
                        current_store.new_with_reduced(blueprint.geode.0, 0, blueprint.geode.1, 0),
                    ),
                })
                .max_by_key(|(r, _)| *r)
                .unwrap_or((0, 0));

            let geodes_if_nothing_built = max_possible_geodes(
                cache,
                blueprint,
                current_time,
                current_bots.clone(),
                current_store.clone(),
            );

            result.1 += geodes_if_nothing_built.1;
            result.0 = std::cmp::max(result.0, geodes_if_nothing_built.0);

            cache.insert(
                (current_time, current_bots.clone(), current_store.clone()),
                result.0,
            );
            return result;
        }

        // println!("Descision point! {current_time}s with {current_bots:?} and {current_store:?} => {possible_robots:?}");
    }

    // time is always 0
    cache.insert(
        (current_time, current_bots.clone(), current_store.clone()),
        current_store.geode,
    );
    (current_store.geode, 1)
}

fn quality_level(blueprint_number: usize, blueprint: &Blueprint) -> u64 {
    blueprint_number as u64 * max_geodes_from_blueprint(blueprint, 24) as u64
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Robots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robots {
    fn new_with_increased(&self, ore: u32, clay: u32, obsidian: u32, geode: u32) -> Robots {
        Self {
            ore: self.ore + ore,
            clay: self.clay + clay,
            obsidian: self.obsidian + obsidian,
            geode: self.geode + geode,
        }
    }

    fn increase(&mut self, robot_to_build: RobotType) {
        match robot_to_build {
            RobotType::Ore => self.ore += 1,
            RobotType::Clay => self.clay += 1,
            RobotType::Obsidian => self.obsidian += 1,
            RobotType::Geode => self.geode += 1,
        }
    }

    fn possible_robots_possible(&self) -> Vec<RobotType> {
        let mut result = Vec::with_capacity(4);
        if self.ore > 0 {
            result.push(RobotType::Ore);
            result.push(RobotType::Clay);
        }
        if self.clay > 0 {
            result.push(RobotType::Obsidian);
        }
        if self.obsidian > 0 {
            result.push(RobotType::Geode);
        }

        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Store {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}
impl Store {
    fn advance(&mut self, current_bots: &Robots) {
        self.ore += current_bots.ore;
        self.clay += current_bots.clay;
        self.obsidian += current_bots.obsidian;
        self.geode += current_bots.geode;
    }

    fn new_with_reduced(&self, ore: u32, clay: u32, obsidian: u32, geode: u32) -> Self {
        Self {
            ore: self.ore - ore,
            clay: self.clay - clay,
            obsidian: self.obsidian - obsidian,
            geode: self.geode - geode,
        }
    }

    fn possible_robots(&self, blueprint: &Blueprint) -> Vec<RobotType> {
        let mut result = Vec::with_capacity(4);
        if self.ore >= blueprint.ore {
            result.push(RobotType::Ore);
        }
        if self.ore >= blueprint.clay {
            result.push(RobotType::Clay);
        }
        if self.ore >= blueprint.obsidian.0 && self.clay >= blueprint.obsidian.1 {
            result.push(RobotType::Obsidian);
        }
        if self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1 {
            result.push(RobotType::Geode);
        }
        result
    }

    fn create(&mut self, robot_to_build: RobotType, blueprint: &Blueprint) {
        match robot_to_build {
            RobotType::Ore => {
                self.ore -= blueprint.ore;
            }
            RobotType::Clay => {
                self.ore -= blueprint.clay;
            }
            RobotType::Obsidian => {
                self.ore -= blueprint.obsidian.0;
                self.clay -= blueprint.obsidian.1;
            }
            RobotType::Geode => {
                self.ore -= blueprint.geode.0;
                self.obsidian -= blueprint.geode.1;
            }
        }
    }
}

impl Default for Robots {
    fn default() -> Self {
        Self {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

fn parse_blueprint(line: &str) -> Result<Blueprint> {
    let numbers = line
        .split_whitespace()
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_numeric()))
        .map(|s| {
            s.parse::<u32>()
                .map_err(|e| eyre!("Cannot parse number :: {e}"))
        })
        .collect::<Result<Vec<_>>>()?;

    let numbers: (u32, u32, u32, u32, u32, u32) = numbers
        .into_iter()
        .next_tuple()
        .ok_or(eyre!("Line does not have 6 numbers!"))?;

    Ok(Blueprint {
        ore: numbers.0,
        clay: numbers.1,
        obsidian: (numbers.2, numbers.3),
        geode: (numbers.4, numbers.5),
    })
}

fn product_of_max(input: &str, remaining: usize) -> Result<u64> {
    Ok(input
        .lines()
        .take(remaining)
        .map(|l| parse_blueprint(l).map(|b| max_geodes_from_blueprint(&b, 32) as u64))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .product())
}

#[cfg(test)]
mod tests {
    use crate::{
        max_geodes_from_blueprint, parse_blueprint, product_of_max, sum_of_quality_levels,
        Blueprint,
    };

    #[test]
    fn max_geodes_test1() {
        let blueprint = Blueprint {
            ore: 4,
            clay: 2,
            obsidian: (3, 14),
            geode: (2, 7),
        };

        assert_eq!(9, max_geodes_from_blueprint(&blueprint, 24));
    }

    #[test]
    fn max_geodes_test2() {
        let blueprint = Blueprint {
            ore: 2,
            clay: 3,
            obsidian: (3, 8),
            geode: (3, 12),
        };

        assert_eq!(12, max_geodes_from_blueprint(&blueprint, 24));
    }

    #[test]
    fn parse_blueprint_test() {
        assert_eq!(
            Blueprint {
                ore: 2,
                clay: 2,
                obsidian: (2, 20),
                geode: (2, 14),
            },
            parse_blueprint("Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 14 obsidian."
).unwrap()
        );
    }

    #[test]
    fn example_part1() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        assert_eq!(33, sum_of_quality_levels(input).unwrap());
    }

    #[test]
    fn example_part2() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        assert_eq!(56 * 62, product_of_max(input, 2).unwrap());
    }
}
