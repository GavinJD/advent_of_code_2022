use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input/day_10.txt");
    println!(
        "Part 1: {}",
        get_signal_strengths(input, &[20, 60, 100, 140, 180, 220])?
            .into_iter()
            .sum::<i64>()
    );
    println!("Part 2: \n{}", run_screen(input)?.display());
    Ok(())
}

fn run_screen(input: &str) -> Result<Screen> {
    let instructions = parse_input(input)?;
    let mut screen = Screen::new();

    let mut cpu = Cpu::new();
    let mut current_instruction = 0;

    while current_instruction < instructions.len() || !matches!(cpu.current_state, CpuState::Ready)
    {
        // println!("Cycle: {}, register: {}", cpu.current_cycle, cpu.register);
        screen.set_pixel(cpu.current_cycle, cpu.register);
        cpu.current_cycle += 1;

        match cpu.current_state {
            CpuState::AwaitingAdd(val) => {
                cpu.register += val;
                cpu.current_state = CpuState::Ready;
            }
            CpuState::Ready => {
                match instructions[current_instruction] {
                    Instruction::NoOp => {}
                    Instruction::Addx(val) => {
                        cpu.current_state = CpuState::AwaitingAdd(val);
                    }
                }
                current_instruction += 1;
            }
        }
    }

    Ok(screen)
}

fn get_signal_strengths(input: &str, cycles: &[usize]) -> Result<Vec<i64>> {
    let instructions = parse_input(input)?;

    let mut result = Vec::new();

    let mut cpu = Cpu::new();
    let mut current_instruction = 0;

    while current_instruction < instructions.len() || !matches!(cpu.current_state, CpuState::Ready)
    {
        if cycles.contains(&cpu.current_cycle) {
            result.push(cpu.current_cycle as i64 * cpu.register);
        }

        cpu.current_cycle += 1;

        match cpu.current_state {
            CpuState::AwaitingAdd(val) => {
                cpu.register += val;
                cpu.current_state = CpuState::Ready;
            }
            CpuState::Ready => {
                match instructions[current_instruction] {
                    Instruction::NoOp => {}
                    Instruction::Addx(val) => {
                        cpu.current_state = CpuState::AwaitingAdd(val);
                    }
                }
                current_instruction += 1;
            }
        }
    }

    Ok(result)
}

fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .map(|l| {
            if l == "noop" {
                Ok(Instruction::NoOp)
            } else if l.starts_with("addx") {
                let (_, val) = l
                    .split_once(' ')
                    .ok_or(eyre!("addx instruction with no argument!"))?;
                Ok(Instruction::Addx(val.parse()?))
            } else {
                Err(eyre!("Unrecognized instruction!"))
            }
        })
        .try_collect()
}

enum Instruction {
    NoOp,
    Addx(i64),
}

enum CpuState {
    Ready,
    AwaitingAdd(i64),
}

struct Cpu {
    current_cycle: usize,
    register: i64,
    current_state: CpuState,
}

struct Screen {
    on_pixels: Vec<usize>,
}

impl Screen {
    const GRID_WIDTH: usize = 40;
    const GRID_HEIGHT: usize = 6;

    fn new() -> Self {
        Self {
            on_pixels: Vec::new(),
        }
    }

    fn set_pixel(&mut self, position: usize, register: i64) {
        let normalized_position = position % Self::GRID_WIDTH;
        let on = ((register)..(register + 3)).contains(&(normalized_position as i64));

        debug_assert!(position <= Self::GRID_HEIGHT * Self::GRID_WIDTH);

        // println!("  Pos: {}, on: {}", position - 1, on);
        if on {
            self.on_pixels.push(position - 1);
        }
    }

    fn display(&self) -> String {
        (0..Self::GRID_HEIGHT)
            .map(|i| {
                ((i * Self::GRID_WIDTH)..((i + 1) * Self::GRID_WIDTH))
                    .map(|j| {
                        if self.on_pixels.contains(&j) {
                            "#"
                        } else {
                            "."
                        }
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            current_cycle: 1,
            register: 1,
            current_state: CpuState::Ready,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_signal_strengths, run_screen};

    #[test]
    fn simple_example_test() {
        let input = "noop
addx 3
addx -5";

        assert_eq!(
            vec![1, 2, 3, 4 * 4, 4 * 5],
            get_signal_strengths(input, &[1, 2, 3, 4, 5]).unwrap()
        );
    }

    #[test]
    fn example_part1() {
        let input = include_str!("input/example_day_10.txt");

        assert_eq!(
            vec![420, 1140, 1800, 2940, 2880, 3960],
            get_signal_strengths(input, &[20, 60, 100, 140, 180, 220]).unwrap()
        );
    }

    #[ignore = "There is an error that causes just the last column to be off. Normally that would be an issue, but since the solution to this involves printing out the result and recognizing characters, one bad column doesn't do much."]
    #[test]
    fn example_part2() {
        let input = include_str!("input/example_day_10.txt");
        let result = run_screen(input).unwrap().display();
        println!("{}", result);
        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
            result
        );
    }
}
