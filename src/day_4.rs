use std::str::FromStr;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    println!("Day 4");
    let input = include_str!("input/day_4.txt");
    println!(
        "Part 1: {}",
        fully_overlapping_pair_count(input, Overlap::Full)?
    );
    println!(
        "Part 2: {}",
        fully_overlapping_pair_count(input, Overlap::Partial)?
    );

    Ok(())
}

fn fully_overlapping_pair_count(
    input: &str,
    overlap_type: Overlap,
) -> color_eyre::eyre::Result<u64> {
    input
        .lines()
        .flat_map(|line| line.split_once(','))
        .map(|(first, second)| -> Result<u64, color_eyre::eyre::Report> {
            let first_pair = Pair::from_str(first)?;
            let second_pair = Pair::from_str(second)?;

            match overlap_type {
                Overlap::Full => Ok((first_pair.contains(&second_pair)
                    || second_pair.contains(&first_pair))
                .into()),
                Overlap::Partial => Ok(first_pair.overlaps(&second_pair).into()),
            }
        })
        .sum()
}

enum Overlap {
    Partial,
    Full,
}

struct Pair(u64, u64);

impl FromStr for Pair {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once('-') {
            let start = start.parse::<u64>()?;
            let end = end.parse::<u64>()?;

            Ok(Pair(start, end))
        } else {
            Err(color_eyre::eyre::eyre!("Cannot parse pair from string"))
        }
    }
}

impl Pair {
    fn contains(&self, other: &Pair) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Pair) -> bool {
        // other.0 >= self.0 && other.0 <= self.1 || other.1 >= self.0 && other.0 <= self.1 || other.contains(self)
        // clippy told this, above is the naive expression
        (other.1 >= self.0 || other.0 >= self.0) && other.0 <= self.1 || other.contains(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{fully_overlapping_pair_count, Overlap};

    #[test]
    fn example_part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(
            fully_overlapping_pair_count(input, Overlap::Full).expect("example should work"),
            2
        );
    }
    #[test]
    fn example_part2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(
            fully_overlapping_pair_count(input, Overlap::Partial).expect("example should work"),
            4
        );
    }
}
