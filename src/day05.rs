use crate::load_input;
use regex::Regex;

fn get_stacks(lines: &Vec<&str>) -> Vec<Vec<char>> {
    let height = lines.iter().position(|&l| l == "").unwrap() - 1;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    for crate_row in &lines[..height] {
        let mut row_chars = crate_row.chars();
        row_chars.nth(0).unwrap();
        for i in 0..9 {
            let c = row_chars.nth(0).unwrap();
            if c != ' ' {
                stacks[i].insert(0, c);
            }
            row_chars.nth(2);
        }
    }
    stacks
}

fn get_instructions(lines: &Vec<&str>) -> Vec<[usize; 3]> {
    let mut instructions: Vec<[usize; 3]> = Vec::new();
    let first_line = lines.iter().position(|&l| l.contains("move")).unwrap();
    let regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for inst_str in &lines[first_line..] {
        let caps = regex.captures(inst_str).unwrap();
        instructions.push([
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
        ]);
    }
    instructions
}

fn get_stack_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut tops = String::new();
    for stack in stacks {
        let len = stack.len();
        tops.push(stack[len - 1]);
    }
    tops
}

pub fn run() {
    println!("DAY 5:");

    let contents = load_input("data/day05.txt");
    let mut lines: Vec<&str> = contents.split("\n").collect();
    lines.pop();

    let mut stacks1 = get_stacks(&lines);
    let mut stacks2 = stacks1.clone();
    let instructions = get_instructions(&lines);

    for instruction in instructions {
        let n = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        // TASK 1
        for _ in 0..n {
            let crate_ = stacks1[from].pop().unwrap();
            stacks1[to].push(crate_);
        }
        // TASK 2
        let idx = stacks2[from].len() - n;
        let mut crates: Vec<char> = stacks2[from].drain(idx..).collect();
        stacks2[to].append(&mut crates);
    }

    println!("TASK 1: {}", get_stack_tops(&stacks1));
    println!("TASK 2: {}", get_stack_tops(&stacks2));
}
