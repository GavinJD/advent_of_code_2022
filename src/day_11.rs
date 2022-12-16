use std::collections::VecDeque;

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space1},
    combinator::{map, value},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_11.txt");
    println!(
        "Part 1: {}",
        monkey_business(&input.replace("\r\n", "\n"), 20)?
    );
    println!(
        "Part 2: {}",
        monkey_business(&input.replace("\r\n", "\n"), 10000)?
    );
    Ok(())
}

fn monkey_business(input: &str, rounds: usize) -> Result<u64> {
    let mut monkeys = parse_monkeys(input)?;
    monkeys.sort_by(|m1, m2| Ord::cmp(&m1.id, &m2.id));
    let magic_monkey_number: u64 = monkeys.iter().map(|m| m.divisible_test_number).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop_front().unwrap();
                let to_toss_monkey = monkeys[i].find_to_toss_monkey(rounds, magic_monkey_number, i, &mut item);
                monkeys[to_toss_monkey].items.push_back(item);
                monkeys[i].inspect_count += 1;
            }
        }

        // println!("After round {}", round);
        // for (i, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}: {}", i, monkey.items.iter().map(u64::to_string).join(", "));
        // }
        // println!()
        // for monkey in &monkeys {
        //     for item in &monkey.items {
        //         println!(
        //             "Monkey {} : item {}, visited: {}",
        //             monkey.id,
        //             item.val,
        //             item.visited_ids.iter().map(u64::to_string).join(",")
        //         );
        //     }
        // }
    }

    Ok(monkeys
        .iter()
        .map(|m| m.inspect_count)
        .sorted_by_key(|&n| std::cmp::Reverse(n))
        .take(2)
        .product())
}

fn parse_monkeys(input: &str) -> Result<Vec<Monkey>> {
    input
        .split("\n\n")
        .map(parse_monkey)
        .collect::<Result<Vec<_>>>()
}

fn parse_monkey(s: &str) -> Result<Monkey> {
    let (_, (id, items, operation, (divisible_test_number, test_true_monkey, test_false_monkey))) =
        tuple((
            terminated(monkey_heading, newline),
            terminated(starting_items, newline),
            terminated(operation, newline),
            test_parse,
        ))(s)
        .map_err(|e| eyre!("Could not parse :: {}", e))?;
    Ok(Monkey {
        id,
        items,
        operation,
        divisible_test_number,
        test_true_monkey,
        test_false_monkey,
        inspect_count: 0,
    })
}

fn monkey_heading(input: &str) -> IResult<&str, u64> {
    terminated(
        preceded(tag("Monkey "), nom::character::complete::u64),
        tag(":"),
    )(input)
}
fn starting_items(input: &str) -> IResult<&str, VecDeque<MonkeyItem>> {
    preceded(
        tag("  Starting items: "),
        map(
            separated_list0(
                tag(", "),
                map(nom::character::complete::u64, MonkeyItem::from),
            ),
            VecDeque::from,
        ),
    )(input)
}
fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, (lhs, operation_type, rhs)) = preceded(
        tag("  Operation: new = "),
        tuple((
            operand,
            delimited(
                space1,
                alt((
                    value(OperationType::Add, tag("+")),
                    value(OperationType::Multiply, tag("*")),
                )),
                space1,
            ),
            operand,
        )),
    )(input)?;
    Ok((
        input,
        Operation {
            lhs,
            rhs,
            operation_type,
        },
    ))
}
fn operand(input: &str) -> IResult<&str, Operand> {
    alt((
        value(Operand::Input, tag("old")),
        map(nom::character::complete::u64, Operand::Number),
    ))(input)
}
fn test_parse(input: &str) -> IResult<&str, (u64, u64, u64)> {
    tuple((
        terminated(
            preceded(tag("  Test: divisible by "), nom::character::complete::u64),
            newline,
        ),
        terminated(
            preceded(
                tag("    If true: throw to monkey "),
                nom::character::complete::u64,
            ),
            newline,
        ),
        preceded(
            tag("    If false: throw to monkey "),
            nom::character::complete::u64,
        ),
    ))(input)
}

#[derive(PartialEq, Eq, Debug)]
struct Monkey {
    id: u64,
    items: VecDeque<MonkeyItem>,
    operation: Operation,
    divisible_test_number: u64,
    test_true_monkey: u64,
    test_false_monkey: u64,
    inspect_count: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct MonkeyItem {
    val: u64,
    visited_ids: Vec<u64>,
    current_visited_id: usize,
}

impl From<u64> for MonkeyItem {
    fn from(n: u64) -> Self {
        MonkeyItem {
            val: n,
            visited_ids: Vec::new(),
            current_visited_id: 0,
        }
    }
}

impl Monkey {
    pub(crate) fn find_to_toss_monkey(
        &self,
        rounds: usize,
        magic_monkey_number: u64,
        current_monkey: usize,
        item: &mut MonkeyItem,
    ) -> usize {
        let is_big = rounds == 10000;

        // if is_big && item.visited_ids.len() > 7 && is_cycle(&item.visited_ids) {
        //     item.current_visited_id = (item.current_visited_id + 1) % item.visited_ids.len();
        //     return item.visited_ids[(item.current_visited_id + 1) % item.visited_ids.len()]
        //         as usize;
        // }

        let score = if !is_big {
            (self.operation.apply_operation(item.val) / 3 ) % (magic_monkey_number)
        } else {
            self.operation.apply_operation(item.val) % (magic_monkey_number)
        };

        item.val = score;
        item.visited_ids.push(current_monkey as u64);
        item.current_visited_id = item.visited_ids.len() - 1;

        if score % self.divisible_test_number == 0 {
            self.test_true_monkey as usize
        } else {
            self.test_false_monkey as usize
        }
    }
}

// fn is_cycle(visited_ids: &[u64]) -> bool {
//     let mut is_cycle = false;
//     for i in (0..(visited_ids.len() - 1)).rev() {
//         if visited_ids[i] == visited_ids[visited_ids.len() - 1] {
//             is_cycle = true;
//             break;
//         }
//     }
//
//     println!("{} is {} cycle", visited_ids.iter().map(u64::to_string).join(","), is_cycle);
//     is_cycle
// }

#[derive(PartialEq, Eq, Debug)]
struct Operation {
    lhs: Operand,
    rhs: Operand,
    operation_type: OperationType,
}
impl Operation {
    fn apply_operation(&self, item: u64) -> u64 {
        let l = match self.lhs {
            Operand::Input => item,
            Operand::Number(val) => val,
        };
        let r = match self.rhs {
            Operand::Input => item,
            Operand::Number(val) => val,
        };
        match self.operation_type {
            OperationType::Add => l + r,
            OperationType::Multiply => l * r,
        }
    }
}
#[derive(PartialEq, Eq, Debug, Clone)]
enum Operand {
    Input,
    Number(u64),
}
#[derive(PartialEq, Eq, Debug, Clone)]
enum OperationType {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{monkey_business, parse_monkeys, Monkey, Operand, Operation, OperationType};

    #[test]
    fn parse_monkey_test() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";
        assert_eq!(
            vec![
                Monkey {
                    id: 0,
                    items: VecDeque::from([79u64.into(), 98.into()]),
                    operation: Operation {
                        lhs: Operand::Input,
                        rhs: Operand::Number(19),
                        operation_type: OperationType::Multiply,
                    },
                    divisible_test_number: 23,
                    test_true_monkey: 2,
                    test_false_monkey: 3,
                    inspect_count: 0,
                },
                Monkey {
                    id: 1,
                    items: VecDeque::from([54.into(), 65.into(), 75.into(), 74.into()]),
                    operation: Operation {
                        lhs: Operand::Input,
                        rhs: Operand::Number(6),
                        operation_type: OperationType::Add,
                    },
                    divisible_test_number: 19,
                    test_true_monkey: 2,
                    test_false_monkey: 0,
                    inspect_count: 0
                }
            ],
            parse_monkeys(input).unwrap()
        );
    }

    #[test]
    fn example_part1() {
        let input = include_str!("input/example_day_11.txt").replace("\r\n", "\n");
        assert_eq!(10605, monkey_business(&input, 20).unwrap());
    }

    #[test]
    fn example_part2() {
        let input = include_str!("input/example_day_11.txt").replace("\r\n", "\n");
        assert_eq!(2713310158, monkey_business(&input, 10000).unwrap());
    }
}
