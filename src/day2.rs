use nom::character::complete::space1;
use nom::sequence::separated_pair;
use nom::{bytes::streaming::tag, combinator::value, IResult};

#[derive(Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum Strategy {
    Win,
    Lose,
    Draw,
}

fn rps(input: &str) -> IResult<&str, RPS> {
    nom::branch::alt((
        value(RPS::Rock, tag("A")),
        value(RPS::Paper, tag("B")),
        value(RPS::Scissors, tag("C")),
    ))(input)
}

fn strat(input: &str) -> IResult<&str, Strategy> {
    nom::branch::alt((
        value(Strategy::Lose, tag("X")),
        value(Strategy::Draw, tag("Y")),
        value(Strategy::Win, tag("Z")),
    ))(input)
}

fn map_strat_to_rps(other: &RPS, strat: &Strategy) -> RPS {
    match strat {
        Strategy::Win => match other {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        },
        Strategy::Lose => match other {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        },
        Strategy::Draw => other.clone(),
    }
}

// answer: 16098
pub fn day2(file_path: &str) {
    let contents = std::fs::read_to_string(file_path).unwrap();

    let score = contents.lines().fold(0, |acc, line| {
        let (_, (elf, strat)) = separated_pair(rps, space1, strat)(line).unwrap();
        let me = map_strat_to_rps(&elf, &strat);

        let type_points = match me {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        let match_points = match (elf, me) {
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => 0,
            (RPS::Scissors, RPS::Rock) | (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) => 6,
            _ => 3,
        };

        acc + type_points + match_points
    });

    println!("{:?}", score);
}
