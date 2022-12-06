use crate::load_input;
use std::collections::HashSet;

fn check_unique(letters: &Vec<char>) -> bool {
    let len = letters.len();
    letters
        .iter()
        .collect::<HashSet<&char>>()
        .into_iter()
        .collect::<Vec<&char>>()
        .len()
        == len
}

pub fn run() {
    println!("DAY 6:");
    let mut curr_four: Vec<char> = Vec::new();
    let mut found_four = false;
    let mut curr_fourteen: Vec<char> = Vec::new();
    let mut found_fourteen = false;
    let mut position1 = 0;
    let mut position2 = 0;
    for c in load_input("data/day06.txt").as_str().chars() {
        if c != '\n' {
            if !found_four && position1 >= 4 {
                if check_unique(&curr_four) {
                    found_four = true;
                }
            }
            if !found_fourteen && position2 >= 14 {
                if check_unique(&curr_fourteen) {
                    found_fourteen = true;
                }
            }
            if found_four && found_fourteen {
                break;
            }
            if !found_four {
                position1 += 1;
                curr_four.insert(0, c);
                if curr_four.len() == 5 {
                    curr_four.pop();
                }
            }
            if !found_fourteen {
                position2 += 1;
                curr_fourteen.insert(0, c);
                if curr_fourteen.len() == 15 {
                    curr_fourteen.pop();
                }
            }
        }
    }
    println!("TASK 1: {}", position1);
    println!("TASK 2: {}", position2);
}
