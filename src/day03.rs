use crate::load_input;
use std::collections::HashSet;

fn get_priority(letter: char) -> u32 {
    let ascii = letter as u32;
    match ascii {
        97..=122 => ascii - 96,
        65..=90 => ascii - 38,
        _ => panic!("Oops"),
    }
}

pub fn run() {
    println!("DAY 3:");
    let contents: &str = &load_input("data/day03.txt");
    let mut rucksacks: Vec<&str> = contents.split("\n").collect();
    rucksacks.pop(); // get rid of last (empty) element

    // TASK 1
    let mut total1: u32 = 0;
    for rucksack in rucksacks.iter() {
        let length = rucksack.len();
        let comp1 = &rucksack[..length / 2];
        let comp1_set: HashSet<char> = comp1.chars().collect();
        let comp2 = &rucksack[length / 2..];
        for c in comp2.chars() {
            if comp1_set.contains(&c) {
                total1 += get_priority(c);
                break;
            }
        }
    }
    println!("TASK 1: {}", total1);

    // TASK 2
    let mut total2: u32 = 0;
    for i in 0..(rucksacks.len() / 3) {
        let ruck1 = &rucksacks[3 * i];
        let ruck2 = &rucksacks[3 * i + 1];
        let ruck3 = &rucksacks[3 * i + 2];
        let ruck2_set: HashSet<char> = ruck2.chars().collect();
        let ruck3_set: HashSet<char> = ruck3.chars().collect();
        for c in ruck1.chars() {
            if ruck2_set.contains(&c) && ruck3_set.contains(&c) {
                total2 += get_priority(c);
                break;
            }
        }
    }
    println!("TASK 2: {}", total2);
}
