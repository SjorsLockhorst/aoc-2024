use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_rules(rule_txt: &str) -> HashMap<u32, HashSet<u32>> {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    for rule in rule_txt.lines() {
        let (before, after) = rule.split_once("|").unwrap();
        let all_after = rules
            .entry(before.parse::<u32>().unwrap())
            .or_insert(HashSet::new());
        all_after.insert(after.parse::<u32>().unwrap());
    }
    return rules;
}

fn update_is_ordered(updates: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut disallowed: HashSet<u32> = HashSet::new();

    for update in updates.iter().rev() {
        if disallowed.contains(&update) {
            return false;
        }
        let rule_disallowed = rules.get(&update);
        if rule_disallowed != None {
            for rule in rule_disallowed.unwrap() {
                disallowed.insert(*rule);
            }
        }
    }
    return true
}

fn part_one(contents: String) {
    let (rule_txt, update_txt) = contents.split_once("\n\n").unwrap();
    let rules = parse_rules(rule_txt);

    let mut answer = 0;
    for update_line in update_txt.lines() {
        let updates = update_line
            .split(",")
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if update_is_ordered(&updates, &rules) {
            answer += updates[updates.len() / 2];
        }
    }
    println!("{}", answer);
}

fn part_two(contents: String) {
    let (rule_txt, update_txt) = contents.split_once("\n\n").unwrap();
    let rules = parse_rules(rule_txt);

    let mut answer = 0;
    for update_line in update_txt.lines() {
        let updates = update_line
            .split(",")
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if !update_is_ordered(&updates, &rules) {
            println!("{} update unordered", update_line);
        }
    }
    println!("{}", answer);
}

pub fn main() {
    let test = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let contents = fs::read_to_string("./inputs/day_5.txt").expect("Should be able to find file");
    // part_one(contents);
    part_two(test.to_string());
    
}
