use crate::load_input;

pub fn run() {
    println!("DAY 2:");
    let contents = load_input("data/day02.txt");

    let mut rounds1: Vec<i32> = Vec::new();
    let mut rounds2: Vec<i32> = Vec::new();
    for round in contents.split("\n") {
        if round == String::from("") {
            break;
        }
        let opponent = (round.chars().nth(0).unwrap() as i32) - 65;
        let me = (round.chars().nth(2).unwrap() as i32) - 88;

        // TASK 1
        rounds1.push((me + 1) + 3 * ((me - opponent + 1).rem_euclid(3)));

        // TASK 2
        rounds2.push((me * 3) + ((opponent + me - 1).rem_euclid(3)) + 1)
    }
    println!("TASK 1: {}", rounds1.iter().sum::<i32>());
    println!("TASK 2: {}", rounds2.iter().sum::<i32>());
}
