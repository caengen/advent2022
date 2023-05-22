use std::{
    collections::{HashMap, HashSet},
    path::is_separator,
};

use nom::{character::complete::alpha1, IResult, InputTake};

fn parser(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

// 72348 too high..
// 40344 too high..
pub fn day3(file_path: &str) {
    let contents = std::fs::read_to_string(file_path).unwrap();

    let mut counter: HashMap<char, i32> = HashMap::new();
    let dupes: Vec<char> = contents.lines().fold(Vec::new(), |mut acc, line| {
        let (_, items) = parser(line).unwrap();
        let (first, second) = items.take_split(items.len() / 2);

        let v1: HashSet<_> = first.chars().collect();
        let v2: HashSet<_> = second.chars().collect();

        let mut f = |c: &char| {
            let count = counter.get(&c).unwrap_or(&0);
            counter.insert(*c, count + 1);
        };
        v1.iter().for_each(&mut f);
        v2.iter().for_each(&mut f);

        let dupes: Vec<char> = counter
            .iter()
            .filter(|(_, v)| **v > 1)
            .map(|(k, _)| *k)
            .collect();

        dupes.iter().for_each(|c| {
            acc.push(c.clone());
        });

        counter.clear();
        acc
    });

    let score = dupes.iter().fold(0, |acc, c| {
        let v = *c as u8;

        return if v >= 97 {
            acc + (v - 96) as u32
        } else {
            acc + (v - 38) as u32
        };
    });

    println!("day3 {}", score);
}
