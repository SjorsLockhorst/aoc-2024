use regex::Regex;
use std::fs;

fn part1(contents: String) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    const N: usize = 2;
    let mut total = 0;
    for line in contents.lines() {
        for capture in re.captures_iter(line) {
            let (_, mutliplication): (&str, [&str; N]) = capture.extract();
            let prod: u32 = mutliplication
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .iter()
                .product();
            total += prod;
        }
    }
    println!("{}", total);
}
fn part2(contents: String) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    const N: usize = 2;
    let mut total = 0;

    let augmented_content = vec!["do()", &contents].concat();
    let do_multiplications: Vec<&str> = augmented_content
        .split("don't()")
        .map(|x| x.split_once("do()").unwrap_or(("", "")).1)
        .collect();

    for capture in re.captures_iter(&do_multiplications.concat()) {
        let (_, mutliplication): (&str, [&str; N]) = capture.extract();
        let prod: u64 = mutliplication
            .map(|num_str| num_str.parse::<u64>().unwrap())
            .iter()
            .product();
        total += prod;
    }
    println!("{}", total);
}

// Function to check if a sequence is valid (safe)
pub fn main() {
    let contents = fs::read_to_string("./inputs/day_3.txt").expect("Should be able to find file");
    // let test = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+'mul(32,64](mul(11,8)undo()?mul(8,5))";

    part2(contents);
}
