fn main() {
    let input = include_str!("input/day_2.txt");
    println!("Part 1: {}", score_part1(input));
    println!("Part 2: {}", score_part2(input));
}

fn score_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|t| {
            let pair = t.split_once(' ');

            match pair {
                Some((opponent, player)) => {
                    let player = RPS::from_player(player);
                    let opponent = RPS::from_opponent(opponent);

                    GameResult::calculate_score(player, opponent)
                }
                None => 0,
            }
        })
        .sum()
}

fn score_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|t| {
            let pair = t.split_once(' ');

            let (opponent, result) = pair.unwrap();
            let opponent = RPS::from_opponent(opponent);
            let result = GameResult::from_input(result);
            let player_selected = match (&opponent, result) {
                (RPS::Rock, GameResult::Draw) => RPS::Rock,
                (RPS::Paper, GameResult::Draw) => RPS::Paper,
                (RPS::Scissors, GameResult::Draw) => RPS::Scissors,

                (RPS::Rock, GameResult::Won) => RPS::Paper,
                (RPS::Paper, GameResult::Won) => RPS::Scissors,
                (RPS::Scissors, GameResult::Won) => RPS::Rock,

                (RPS::Scissors, GameResult::Lost) => RPS::Paper,
                (RPS::Rock, GameResult::Lost) => RPS::Scissors,
                (RPS::Paper, GameResult::Lost) => RPS::Rock,
            };

            GameResult::calculate_score(player_selected, opponent)
        })
        .sum()
}

#[derive(PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
enum GameResult {
    Won,
    Lost,
    Draw,
}

impl GameResult {
    fn from_input(s: &str) -> Self {
        match s {
            "X" => Self::Lost,
            "Y" => Self::Draw,
            _ => Self::Won,
        }
    }

    fn calculate_score(player: RPS, opponent: RPS) -> u32 {
        let mut score = match player {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        score += match RPS::did_win(opponent, player) {
            GameResult::Won => 6,
            GameResult::Lost => 0,
            GameResult::Draw => 3,
        };

        score
    }
}

impl RPS {
    fn from_player(player: &str) -> Self {
        match player {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            _ => RPS::Scissors,
        }
    }
    fn from_opponent(opponent: &str) -> Self {
        match opponent {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            _ => RPS::Scissors,
        }
    }

    fn did_win(opponent: RPS, player: RPS) -> GameResult {
        match (opponent, player) {
            (RPS::Rock, RPS::Rock) => GameResult::Draw,
            (RPS::Paper, RPS::Paper) => GameResult::Draw,
            (RPS::Scissors, RPS::Scissors) => GameResult::Draw,

            (RPS::Rock, RPS::Paper) => GameResult::Won,
            (RPS::Paper, RPS::Scissors) => GameResult::Won,
            (RPS::Scissors, RPS::Rock) => GameResult::Won,

            (RPS::Rock, RPS::Scissors) => GameResult::Lost,
            (RPS::Paper, RPS::Rock) => GameResult::Lost,
            (RPS::Scissors, RPS::Paper) => GameResult::Lost,
        }
    }
}

#[test]
fn example_test_part1() {
    let input = "A Y
B X
C Z";

    assert_eq!(score_part1(input), 15);
}

#[test]
fn example_test_part2() {
    let input = "A Y
B X
C Z";

    assert_eq!(score_part2(input), 12);
}
