use crate::load_input;
use regex::Regex;

pub fn run() {
    println!("DAY 1:");
    let contents = load_input("data/day01.txt");
    let contents: &str = &contents[..contents.len() - 1];
    let separator = Regex::new(r"\n\n").unwrap();
    let blocks: Vec<&str> = separator.split(&contents).collect();
    let mut calories_list: Vec<u32> = Vec::new();
    let newline = Regex::new(r"\n").unwrap();
    for block in blocks.iter() {
        let mut cals: u32 = 0;
        for num_str in newline.split(&block) {
            let num_u32 = num_str.parse::<u32>().unwrap();
            cals += num_u32;
        }
        calories_list.push(cals);
    }
    calories_list.sort();
    let length: usize = calories_list.len();
    println!("TASK 1: {}", &calories_list[length - 1]);
    let top_three: u32 = calories_list[length - 3..length].iter().sum();
    println!("TASK 2: {}", top_three);
}
