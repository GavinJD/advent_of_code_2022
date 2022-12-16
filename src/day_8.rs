use std::collections::HashSet;

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_8.txt");
    println!("Part 1: {}", visible_tree_count(input)?);
    println!("Part 2: {}", highest_scenic_score(input)?);
    Ok(())
}

fn visible_tree_count(input: &str) -> Result<u64> {
    let tree_heights = parse_tree_heights(input)?;
    Ok(find_all_visible_trees(&tree_heights).len() as u64)
}

fn find_all_visible_trees(tree_heights: &TreeHeights) -> HashSet<(usize, usize)> {
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    for row in 0..tree_heights.rows {
        let mut current_visible = -1;

        for col in 0..tree_heights.columns {
            if current_visible < tree_heights.heights[row][col] as i32 {
                current_visible = tree_heights.heights[row][col] as i32;
                visible_trees.insert((row, col));
            }
        }
        current_visible = -1;
        for col in (0..tree_heights.columns).rev() {
            if current_visible < tree_heights.heights[row][col] as i32 {
                current_visible = tree_heights.heights[row][col] as i32;
                visible_trees.insert((row, col));
            }
        }
    }

    for col in 0..tree_heights.columns {
        let mut current_visible = -1;
        for row in 0..tree_heights.rows {
            if current_visible < tree_heights.heights[row][col] as i32 {
                current_visible = tree_heights.heights[row][col] as i32;
                visible_trees.insert((row, col));
            }
        }
        current_visible = -1;
        for row in (0..tree_heights.rows).rev() {
            if current_visible < tree_heights.heights[row][col] as i32 {
                current_visible = tree_heights.heights[row][col] as i32;
                visible_trees.insert((row, col));
            }
        }
    }

    // for row in 0..tree_heights.rows {
    //     for col in 0..tree_heights.columns {
    //         if visible_trees.contains(&(row, col)) {
    //             print!("|{}| ", tree_heights.heights[row][col]);
    //         } else {
    //             print!(" {}  ", tree_heights.heights[row][col]);
    //         }
    //     }
    //     println!();
    // }

    visible_trees
}

fn highest_scenic_score(input: &str) -> Result<usize> {
    let tree_heights = parse_tree_heights(input)?;

    (0..tree_heights.rows)
        .flat_map(|r| (0..tree_heights.columns).map(move |c| (r, c)))
        .map(|(r, c)| visibility_score(&tree_heights, r, c))
        .max()
        .ok_or(eyre!("No trees given!"))
}

fn visibility_score(tree_heights: &TreeHeights, trow: usize, tcol: usize) -> usize {
    let tree_height = tree_heights.heights[trow][tcol];
    let visible_ranges = vec![
        (0..trow)
            .rev()
            .find_or_last(|r| tree_heights.heights[*r][tcol] >= tree_height)
            .map(|r| (trow - r))
            .unwrap_or(0),
        ((trow + 1)..tree_heights.rows)
            .find_or_last(|r| tree_heights.heights[*r][tcol] >= tree_height)
            .map(|r| (r - trow))
            .unwrap_or(0),
        (0..tcol)
            .rev()
            .find_or_last(|c| tree_heights.heights[trow][*c] >= tree_height)
            .map(|c| (tcol - c))
            .unwrap_or(0),
        ((tcol + 1)..tree_heights.columns)
            .find_or_last(|c| tree_heights.heights[trow][*c] >= tree_height)
            .map(|c| (c - tcol))
            .unwrap_or(0),
    ];

    // println!("For ({trow},{tcol}) = {tree_height} scores are {visible_ranges:?}, val {}", visible_ranges.iter().product::<usize>());
    visible_ranges.into_iter().product()
}

fn parse_tree_heights(input: &str) -> Result<TreeHeights> {
    let rows = input.lines().count();
    let columns = input.lines().next().map(|l| l.len()).unwrap_or(0);

    let heights = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or(eyre!("Cannot parse number from input"))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(TreeHeights {
        columns,
        rows,
        heights,
    })
}

struct TreeHeights {
    columns: usize,
    rows: usize,
    heights: Vec<Vec<u32>>,
}

#[cfg(test)]
mod tests {
    use crate::{highest_scenic_score, visible_tree_count};

    #[test]
    fn example_part1() {
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(21, visible_tree_count(input).unwrap());
    }

    #[test]
    fn example_part2() {
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(8, highest_scenic_score(input).unwrap());
    }
}
