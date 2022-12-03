use std::num::ParseIntError;

fn main() {
    println!("Day 1");
    let input = include_str!("input/day_1.txt");

    let mut elf_calories = elf_calories(input).unwrap();
    elf_calories.sort();
    elf_calories.reverse();

    let mut total = 0;
    for (i, val) in elf_calories.iter().take(3).enumerate() {
        println!("Top {}: {}", i + 1, val);
        total += val;
    }
    println!("Total top 3: {}", total);
}
// fn max_calories_elf(input: &str) -> Result<i32, ParseIntError> {
//     let mut max = 0;
//     let mut current = 0;
//     for line in input.lines() {
//         if line == "" {
//             if current > max {
//                 max = current;
//             }
//             current = 0;
//         } else {
//             current += line.parse::<i32>()?;
//         }
//     }
//
//     Ok(max)
// }
//
fn elf_calories(input: &str) -> Result<Vec<u32>, ParseIntError> {
    let mut result = Vec::new();
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            result.push(current);
            current = 0;
        } else {
            current += line.parse::<u32>()?;
        }
    }
    Ok(result)
}

#[test]
fn example_test() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    let mut elf_calories = dbg!(elf_calories(input).unwrap());
    elf_calories.sort();
    assert_eq!(elf_calories.last().unwrap().clone(), 24000);
}
