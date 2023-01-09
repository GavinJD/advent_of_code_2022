use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_20.txt");
    // println!("Part 1: {}", visible_tree_count(input)?);
    // println!("Part 2: {}", highest_scenic_score(input)?);
    Ok(())
}

fn mix_data(data: &[i32]) -> Vec<i32> {
    let mut to_mix: Vec<i32> = dbg!(data.to_vec());

    for num in data {
        let (idx, _) = to_mix.iter().find_position(|n| *n == num).unwrap();
        let mut idx = idx as i32;

        for _ in 0..num.abs() {
            let next_idx = idx + num.signum() % data.len() as i32;
            to_mix.swap(
                idx as usize,
                if next_idx < 0 {
                    data.len() next_idx
                } else {
                    next_idx
                },
            );
            idx = next_idx;
        }
        println!("After mixing {} : {:?}", num, to_mix);
    }

    to_mix
}

#[cfg(test)]
mod tests {
    use crate::mix_data;
    #[test]
    fn mix_test() {
        let input = [1, 2, -3, 3, -2, 0, 4];
        assert_eq!(vec![1, 2, -3, 4, 0, 3, -2], mix_data(&input));
    }
}
