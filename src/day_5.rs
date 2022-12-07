use itertools::Itertools;
use std::collections::HashMap;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    println!("Day 5");
    let input = include_str!("input/day_5.txt");
    println!(
        "Part 1: {}",
        top_crates_of_stacks(input, CrateMoverModel::CrateMover9000)?
    );
    println!(
        "Part 2: {}",
        top_crates_of_stacks(input, CrateMoverModel::CrateMover9001)?
    );
    Ok(())
}

fn top_crates_of_stacks(input: &str, model: CrateMoverModel) -> color_eyre::eyre::Result<String> {
    let mut stacks = parse_stacks(input)?;
    let instructions = parse_instructions(input)?;
    for instruction in instructions {
        // dbg!(&instruction);
        // dbg!(&stacks);
        match model {
            CrateMoverModel::CrateMover9000 => {
                for _ in 0..instruction.quantity {
                    let to_move = stacks
                        .0
                        .get_mut(&instruction.start)
                        .ok_or(color_eyre::eyre::eyre!("Start not found"))?;
                    let c = to_move.pop().ok_or(color_eyre::eyre::eyre!(
                        "Start element does not have enough elements"
                    ))?;
                    stacks
                        .0
                        .entry(instruction.end)
                        .or_insert(Vec::new())
                        .push(c);
                }
            }
            CrateMoverModel::CrateMover9001 => {
                let start_stack = stacks
                    .0
                    .get_mut(&instruction.start)
                    .ok_or(color_eyre::eyre::eyre!("Start not found"))?;
                let mut to_move = Vec::with_capacity(instruction.quantity);
                for i in &start_stack[(start_stack.len() - instruction.quantity)..] {
                    to_move.push(i.to_owned());
                }
                start_stack.truncate(start_stack.len() - instruction.quantity);

                let end_stack = stacks.0.entry(instruction.end).or_insert(Vec::new());
                (0..instruction.quantity).for_each(|i| end_stack.push(to_move[i]));
            }
        }
    }

    // dbg!(&stacks);
    Ok(stacks
        .0
        .iter()
        .sorted()
        .filter_map(|(_, stack)| stack.last().cloned())
        .join(""))
}

fn parse_instructions(input: &str) -> color_eyre::eyre::Result<Vec<MoveInstruction>> {
    input
        .lines()
        .filter(|l| l.starts_with("move"))
        .map(|l| {
            let parsed_numbers = l
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();

            Ok(MoveInstruction {
                quantity: *parsed_numbers.first().ok_or(color_eyre::eyre::eyre!(
                    "Move instruction is malformed (1st number not found)"
                ))?,
                start: *parsed_numbers.get(1).ok_or(color_eyre::eyre::eyre!(
                    "Move instruction is malformed (2nd number not found)"
                ))?,
                end: *parsed_numbers.get(2).ok_or(color_eyre::eyre::eyre!(
                    "Move instruction is malformed (3rd number not found)"
                ))?,
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn parse_stacks(input: &str) -> color_eyre::eyre::Result<Stacks> {
    let mut parsed_grid = input
        .lines()
        .skip_while(|l| l.is_empty())
        .take_while(|l| l.trim().starts_with('['))
        .map(|l| -> color_eyre::eyre::Result<_> {
            let mut current_pos = 0;
            let mut h_crates: Vec<Option<Crate>> = Vec::new();
            while current_pos <= l.len() {
                let current_crate = &l[current_pos..(current_pos + 2)];
                if current_crate.trim().is_empty() {
                    current_pos += 4;
                    h_crates.push(None);
                    continue;
                }

                let found_crate = current_crate
                    .chars()
                    .nth(1)
                    .ok_or(color_eyre::eyre::eyre!("Crate is not of format [{{char}}]"))?;
                h_crates.push(Some(found_crate));
                current_pos += 4;
            }

            Ok(h_crates)
        })
        .collect::<Result<Vec<_>, _>>()?;
    parsed_grid.reverse();

    let mut stacks = Stacks(HashMap::new());
    for row in parsed_grid {
        row.iter().enumerate().for_each(|(i, possile_crate)| {
            if let Some(c) = possile_crate {
                stacks.0.entry(i + 1).or_insert(Vec::new()).push(*c);
            }
        })
    }

    Ok(stacks)
}

type Crate = char;

#[derive(PartialEq, Eq, Debug)]
struct Stacks(HashMap<usize, Vec<Crate>>);

#[derive(PartialEq, Eq, Debug)]
struct MoveInstruction {
    quantity: usize,
    start: usize,
    end: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct Procedure {
    stacks: Stacks,
    instructions: Vec<MoveInstruction>,
}

enum CrateMoverModel {
    CrateMover9000,
    CrateMover9001,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_instructions, parse_stacks, top_crates_of_stacks, MoveInstruction, Stacks};

    #[test]
    fn parse_stacks_test() {
        let input = "
[C]    
[Z] [M]
1   2 

move 1 from 2 to 1";

        let mut expected_stack = Stacks(HashMap::new());
        expected_stack.0.insert(1, vec!['Z', 'C']);
        expected_stack.0.insert(2, vec!['M']);

        let parsed_data = parse_stacks(input).unwrap();

        assert_eq!(parsed_data, expected_stack);
    }

    #[test]
    fn parse_instructions_test() {
        let input = "
[C]    
[Z] [M]
1   2 

move 1 from 2 to 1";

        let parsed_data = parse_instructions(input).unwrap();

        assert_eq!(
            parsed_data,
            vec![MoveInstruction {
                quantity: 1,
                start: 2,
                end: 1
            }]
        );
    }

    #[test]
    fn example_part1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(
            top_crates_of_stacks(input, crate::CrateMoverModel::CrateMover9000).unwrap(),
            "CMZ"
        );
    }

    #[test]
    fn example_part2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(
            top_crates_of_stacks(input, crate::CrateMoverModel::CrateMover9001).unwrap(),
            "MCD"
        );
    }
}
