use crate::load_input;
use regex::Regex;

pub fn run() {
    println!("DAY 4:");
    let contents: &str = &load_input("data/day04.txt");
    let mut lines: Vec<&str> = contents.split("\n").collect();
    lines.pop();

    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    let left_regex = Regex::new(r"(\d+)-").unwrap();
    let right_regex = Regex::new(r"-(\d+)").unwrap();
    for line in lines.iter() {
        let mut lefts: Vec<u32> = Vec::new();
        for left_capture in left_regex.captures_iter(line) {
            lefts.push(left_capture[1].parse::<u32>().unwrap());
        }
        let mut rights: Vec<u32> = Vec::new();
        for right_capture in right_regex.captures_iter(line) {
            rights.push(right_capture[1].parse::<u32>().unwrap());
        }

        if (lefts[0] >= lefts[1] && rights[0] <= rights[1])
            || (lefts[0] <= lefts[1] && rights[0] >= rights[1])
        {
            total1 += 1;
        }

        if (lefts[0] >= lefts[1] && lefts[0] <= rights[1])
            || (lefts[1] >= lefts[0] && rights[0] >= lefts[1])
        {
            total2 += 1;
        }
    }
    println!("TASK 1: {}", total1);
    println!("TASK 2: {}", total2);
}
