use nom::{character::complete::digit1, combinator::map_res, number::complete::i32, IResult};
use std::{env, time::Instant};
mod day1;
mod day2;
use day1::*;
use day2::*;

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn dummy(a: &str) {
    println!("dummy");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let res = parse_int(&args[1]);
    let file_path = &args[2];

    if let Ok(("", n)) = res {
        let solve_fn = match n {
            1 => day1_part2,
            2 => day2,
            _ => dummy,
        };
        let start = Instant::now();
        solve_fn(file_path);
        let duration = start.elapsed();
        println!("Execution time for solution is: {:?}", duration);
    }
}
