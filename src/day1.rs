use nom::{
    bytes::complete::take_while, character::complete::digit1, combinator::map_res,
    sequence::terminated, IResult,
};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf(pub Vec<u32>);
impl Elf {
    pub fn total(&self) -> u32 {
        self.0.iter().sum()
    }
}

fn parser(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

// answer 67658
pub fn day1(file_path: &str) {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let (total, _) = contents.lines().fold((0, 0), |mut acc, line| {
        let res = parser(line);
        if let Ok((_, amnt)) = res {
            acc.1 += amnt;
        } else {
            if acc.1 > acc.0 {
                acc.0 = acc.1;
            }
            acc.1 = 0;
        }

        acc
    });

    println!("{:?}", total);
}

pub fn day1_part2(file_path: &str) {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let (mut total, _) = contents.lines().fold((Vec::new(), 0), |mut acc, line| {
        let res = parser(line);
        if let Ok((_, amnt)) = res {
            acc.1 += amnt;
        } else {
            acc.0.push(acc.1);
            acc.1 = 0;
        }

        acc
    });
    total.sort();
    let sum_of_top_3: u32 = total.iter().rev().take(3).sum();

    println!("{:?}", sum_of_top_3);
}
