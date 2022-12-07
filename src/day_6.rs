use itertools::Itertools;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    println!("Day 6");
    let input = include_str!("input/day_6.txt");
    println!(
        "Part 1: {}",
        first_marker_location(input, Marker::PacketMarker)
            .ok_or(color_eyre::eyre::eyre!("No marker found"))?
    );
    println!(
        "Part 1: {}",
        first_marker_location(input, Marker::MessageMarker)
            .ok_or(color_eyre::eyre::eyre!("No marker found"))?
    );
    Ok(())
}

fn first_marker_location(input: &str, marker: Marker) -> Option<usize> {
    let size = match marker {
        Marker::PacketMarker => 4,
        Marker::MessageMarker => 14,
    };
    (0..(input.len() - size))
        .find(|i| input.chars().skip(*i).take(size).all_unique())
        .map(|i| i + size)
}

enum Marker {
    PacketMarker,
    MessageMarker,
}

#[cfg(test)]
mod tests {
    use crate::{first_marker_location, Marker};

    #[test]
    fn example_part1() {
        assert_eq!(
            first_marker_location("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Marker::PacketMarker),
            Some(7)
        );
        assert_eq!(
            first_marker_location("bvwbjplbgvbhsrlpgdmjqwftvncz", Marker::PacketMarker),
            Some(5)
        );
        assert_eq!(
            first_marker_location("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Marker::PacketMarker),
            Some(10)
        );
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            first_marker_location("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Marker::MessageMarker),
            Some(19)
        );
        assert_eq!(
            first_marker_location("bvwbjplbgvbhsrlpgdmjqwftvncz", Marker::MessageMarker),
            Some(23)
        );
        assert_eq!(
            first_marker_location("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Marker::MessageMarker),
            Some(29)
        );
    }
}
