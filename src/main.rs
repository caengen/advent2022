use nom::{character::complete::digit1, combinator::map_res, number::complete::i32, IResult};
use std::env;
mod day1;
use day1::*;

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let res = parse_int(&args[1]);
    let file_path = &args[2];

    if let Ok(("", n)) = res {
        match n {
            1 => day1(file_path),
            _ => {}
        }
    }
}
