use std::{collections::HashMap, fs};

pub fn main() {
    let contents = fs::read_to_string("./inputs/day_1.txt").expect("Should be able to find file");
    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in contents.lines() {
        let (first_num, second_num) = line.split_once("   ").unwrap();
        first.push(first_num.parse::<u32>().unwrap());
        second.push(second_num.parse::<u32>().unwrap());
    }

    first.sort();
    second.sort();

    let mut total = 0;
    let mut occurences = HashMap::new();

    for (first_num, second_num) in first.iter().zip(second.iter()) {
        let diff = first_num.abs_diff(*second_num);
        total += diff;
        *occurences.entry(second_num).or_insert(0) += 1;
    }
    print!("Answer first part {}\n", total);

    let mut similarity_score = 0;
    for first_num in first.iter() {
        let n_occurences = occurences.get(first_num).unwrap_or(&0);
        similarity_score += n_occurences * first_num;
    }
    print!("Answer second part {}\n", similarity_score);

}
