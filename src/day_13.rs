use std::{cmp::Ordering, str::FromStr};

use color_eyre::{
    eyre::{eyre, Result},
    Report,
};
use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, error::Error, multi::separated_list1,
    sequence::delimited, Finish, IResult,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_13.txt");
    println!(
        "Part 1: {}",
        indexes_in_right_order(input)?.iter().sum::<usize>()
    );
    println!("Part 2: {}", decoder_key(input)?);
    Ok(())
}

fn decoder_key(input: &str) -> Result<usize> {
    let divider_packets = [
        Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]),
    ];

    let sorted_packets = divider_packets
        .clone()
        .map(Ok)
        .into_iter()
        .chain(
            input
                .lines()
                .filter(|l| !l.is_empty())
                .map(Packet::from_str),
        )
        .sorted_by(|p1, p2| {
            if p1.is_ok() && p2.is_ok() {
                match p1.as_ref().unwrap().compare(p2.as_ref().unwrap()) {
                    PacketComparison::RightOrder => Ordering::Less,
                    PacketComparison::Undecided => unreachable!(),
                    PacketComparison::WrongOrder => Ordering::Greater,
                }
            } else {
                Ordering::Equal // if its broke, dont fix it
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(divider_packets
        .iter()
        .filter_map(|d| sorted_packets.iter().find_position(|p| *p == d))
        .map(|(i, _)| i + 1)
        .product())
}

fn indexes_in_right_order(input: &str) -> Result<Vec<usize>> {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|pair| {
            let (first, second) = pair
                .split_once('\n')
                .ok_or(eyre!("Found just one packet separated by spaces?"))?;
            // println!("First: {first}, second: {second}");
            let first = Packet::from_str(first)?;
            let second = Packet::from_str(second)?;
            Ok(first.compare(&second))
        })
        .enumerate()
        .filter_map(|(i, comparison)| match comparison {
            Ok(packet_comparison) => {
                if let PacketComparison::RightOrder = packet_comparison {
                    Some(Ok(i + 1))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<Vec<_>>>()
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Packet {
    Integer(u64),
    List(Vec<Packet>),
}
enum PacketComparison {
    Undecided,
    RightOrder,
    WrongOrder,
}

impl Packet {
    fn compare(&self, other: &Packet) -> PacketComparison {
        // println!("Comparing {self:?}, {other:?}");
        match (self, other) {
            // If both values are integers, the lower integer should come first. If the left
            // integer is lower than the right integer, the inputs are in the right order.
            // If the left integer is higher than the right integer, the inputs are not in
            // the right order. Otherwise, the inputs are the same integer; continue checking
            // the next part of the input.
            (Packet::Integer(a), Packet::Integer(b)) => match a.cmp(b) {
                Ordering::Less => PacketComparison::RightOrder,
                Ordering::Equal => PacketComparison::Undecided,
                Ordering::Greater => PacketComparison::WrongOrder,
            },
            // If both values are lists, compare the first value of each list, then the second
            // value, and so on. If the left list runs out of items first, the inputs are in
            // the right order. If the right list runs out of items first, the inputs are
            // not in the right order. If the lists are the same length and no comparison
            // makes a decision about the order, continue checking the next part of the input.
            (Packet::List(list_a), Packet::List(list_b)) => list_a
                .iter()
                .zip(list_b.iter())
                .find_map(|(a, b)| {
                    let compare = a.compare(b);
                    if let PacketComparison::Undecided = compare {
                        None
                    } else {
                        Some(compare)
                    }
                })
                .unwrap_or_else(|| {
                    Packet::Integer(list_a.len() as u64)
                        .compare(&Packet::Integer(list_b.len() as u64))
                }),

            // If exactly one value is an integer, convert the integer to a list which
            // contains that integer as its only value, then retry the comparison. For
            // example, if comparing [0,0,0] and 2, convert the right value to [2] (a list
            // containing 2); the result is then found by instead comparing [0,0,0] and [2]
            (Packet::Integer(a), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*a)]).compare(other)
            }
            (Packet::List(_), Packet::Integer(b)) => {
                self.compare(&Packet::List(vec![Packet::Integer(*b)]))
            }
        }
    }
}
impl FromStr for Packet {
    type Err = Report;
    fn from_str(input: &str) -> Result<Self> {
        parse_packet(input)
            .finish()
            .map(|(_, p)| p)
            .map_err(|e| eyre!("Parsing Error!\n{}\n\n", e))
    }
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    alt((
        map(tag::<&str, &str, Error<_>>("[]"), |_| {
            Packet::List(Vec::new())
        }),
        map(nom::character::complete::u64, Packet::Integer),
        delimited(
            tag("["),
            map(separated_list1(tag(","), parse_packet), Packet::List),
            tag("]"),
        ),
    ))(s)
}

#[cfg(test)]
mod tests {
    use crate::{indexes_in_right_order, decoder_key};

    #[test]
    fn example_part1() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(vec![1, 2, 4, 6], indexes_in_right_order(input).unwrap());
    }


    #[test]
    fn example_part2() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(140, decoder_key(input).unwrap());
    }
}
