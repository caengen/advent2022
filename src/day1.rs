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
// fn elf(input: &str) -> IResult<> {
//   take_while(cond)

pub fn day1(file_path: &str) {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut elfs: Vec<Elf> = Vec::new();
    elfs.push(Elf(vec![]));
    for line in contents.lines() {
        let res = parser(line);
        if let Ok(("", amnt)) = res {
            elfs.last_mut().unwrap().0.push(amnt);
        } else {
            elfs.push(Elf(vec![]));
        }
    }

    let res = elfs
        .iter()
        .reduce(|a, b| if a.total() > b.total() { a } else { b });

    let total = res.unwrap().total();
    println!("{:?}", total);
}
