use std::fs;


pub fn main() {
    let contents = fs::read_to_string("./inputs/day_2.txt").expect("Should be able to find file");

    let mut total_safe = 0;
    for line in contents.lines() {
        let numbers: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut is_safe = true;

        let mut last_diff = 0;
        for i in 0..numbers.len() - 1 {
            let cur = numbers[i];
            let next = numbers[i + 1];
            let diff = cur - next;

            if (diff.abs() < 1) || (diff.abs() > 3) || (last_diff * diff < 0) {
                is_safe = false;
                break;
            }
            last_diff = diff;
        }
        if is_safe {
            total_safe += 1;
        }
    }
    print!("{}\n", total_safe);
}
