use nom::character::complete::{anychar, space1};
use nom::sequence::separated_pair;
use nom::{bytes::streaming::tag, combinator::value, IResult};

fn parse_char(input: &str) -> IResult<&str, char> {
    anychar(input)
}

// answer: 16098
pub fn day2(file_path: &str) {
    let contents = std::fs::read_to_string(file_path).unwrap();

    let score = contents.lines().fold(0, |acc, line| {
        let (_, (a, b)) = separated_pair(parse_char, space1, parse_char)(line).unwrap();
        let a = a as u8;
        let b = b;

        let b = match b {
            'X' => match a {
                // Should lose
                0x41 => 0x43,
                0x42 => 0x41,
                _ => 0x42,
            },
            'Y' => match a {
                // should draw
                _ => a,
            },
            _ => ((a - 0x41 + 0x1) % 0x3) + 0x41, // should win, wrap around a
        };

        let mp: u8 = match a >= b {
            true => match a - b {
                1 => 0,
                2 => 6,
                _ => 3,
            },
            false => match b - a {
                1 => 6,
                2 => 0,
                _ => 3,
            },
        };

        acc + ((b - 0x40) + mp) as u32
    });

    println!("Solution: {:?}", score);
}
