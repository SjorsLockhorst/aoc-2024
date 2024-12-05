use std::fs;

// Function to check if a sequence is valid (safe)
fn is_safe(numbers: &Vec<i32>) -> bool {
    for i in 0..(numbers.len() - 1) {
        let diff = numbers[i + 1] - numbers[i];
        if diff.abs() < 1 || diff.abs() > 3 || (i > 0 && diff.signum() != (numbers[i] - numbers[i - 1]).signum()) {
            return false;
        }
    }
    true
}

pub fn main() {
    let contents = fs::read_to_string("./inputs/day_2.txt").expect("Should be able to find file");

    let mut total_safe = 0;

    for line in contents.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
                                    .map(|x| x.parse::<i32>().unwrap())
                                    .collect();

        // Check if the report is already safe
        if is_safe(&numbers) {
            total_safe += 1;
            continue;
        }

        // Try removing one level at a time
        for i in 0..numbers.len() {
            let mut modified_numbers = numbers.clone();
            modified_numbers.remove(i);
            if is_safe(&modified_numbers) {
                total_safe += 1;
                break;
            }
        }
    }

    println!("{}", total_safe);
}
