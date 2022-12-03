fn main() {
    println!("Day 3");
    let input = include_str!("input/day_3.txt");
    println!("Part 1: {}", sum_of_common_priorities(input));
    println!("Part 2: {}", sum_of_elf_group_priorities(input));
}

fn sum_of_common_priorities(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let common = find_common_for_2(first, second).unwrap();
            char_priority(common)
        })
        .sum()
}

fn sum_of_elf_group_priorities(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total = 0;
    for i in (0..lines.len()).step_by(3) {
        let common = find_common_for_3(lines[i], lines[i + 1], lines[i + 2]).unwrap();
        total += char_priority(common)
    }

    total
}

fn find_first_common_char(strings: &[&str]) -> Option<char> {
    strings.first().and_then(|first| {
        first
            .chars()
            .find(|fchar| strings.iter().all(|s| s.find(*fchar).is_some()))
    })
}

fn find_common_for_2(first: &str, second: &str) -> Option<char> {
    find_first_common_char(&[first, second])
}

fn find_common_for_3(first: &str, second: &str, third: &str) -> Option<char> {
    find_first_common_char(&[first, second, third])
}

fn char_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        char_priority, find_common_for_2, sum_of_common_priorities, sum_of_elf_group_priorities,
    };

    #[test]
    fn find_common_test() {
        assert_eq!(find_common_for_2("abcdef", "pqrsxyza"), Some('a'));
        assert_eq!(find_common_for_2("vJrwpWtwJgWr", "hcsFMMfFFhFp"), Some('p'));
        assert_eq!(
            find_common_for_2("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            Some('L')
        );
    }

    #[test]
    fn char_priority_test() {
        assert_eq!(char_priority('a'), 1);
        assert_eq!(char_priority('z'), 26);
        assert_eq!(char_priority('A'), 27);
        assert_eq!(char_priority('Z'), 52);
    }

    #[test]
    fn example_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(sum_of_common_priorities(input), 157);
    }

    #[test]
    fn example_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(sum_of_elf_group_priorities(input), 70);
    }
}
