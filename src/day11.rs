use crate::load_input;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Sq,
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test_div: u64,
    pub true_idx: usize,
    pub false_idx: usize,
    pub inspections: u64,
}

impl Monkey {
    pub fn update_worry(&self, init_worry: u64) -> u64 {
        match self.operation {
            Operation::Add(x) => init_worry + x,
            Operation::Mul(x) => init_worry * x,
            Operation::Sq => init_worry.pow(2),
        }
    }

    pub fn test_worry(&self, worry: u64) -> usize {
        if worry % self.test_div == 0 {
            self.true_idx
        } else {
            self.false_idx
        }
    }
}

fn get_number_from_line(line: &str) -> u64 {
    lazy_static! {
        static ref NUMREGEX: Regex = Regex::new(r"\d+").unwrap();
    }
    u64::from_str_radix(NUMREGEX.find(line).unwrap().as_str(), 10).unwrap()
}

fn get_monkeys(input: &String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let monkeys_info = input.split(&"\n\n");
    let num_regex = Regex::new(r"\d+").unwrap();
    let op_regex = Regex::new(r"old (\+|\*) (old|\d+)").unwrap();
    for monkey_info in monkeys_info {
        let mut monkey_info_lines = monkey_info.lines();
        // First line: Monkey <n>:
        // This is useless. Discard
        monkey_info_lines.next().unwrap();
        // Second line: Starting items: a, b, ..., z
        // Extract numbers into a vec, and reverse
        let mut start_items: Vec<u64> = num_regex
            .find_iter(monkey_info_lines.next().unwrap())
            .map(|s| u64::from_str_radix(s.as_str(), 10).unwrap())
            .collect::<Vec<u64>>();
        start_items.reverse();
        // Third line: Operation: new = old <operation> <operand>
        let op_cap = op_regex
            .captures(monkey_info_lines.next().unwrap())
            .unwrap();

        let operation: Operation;
        let operand = op_cap.get(2).unwrap().as_str();
        if operand == "old" {
            operation = Operation::Sq;
        } else {
            let operator = op_cap.get(1).unwrap().as_str();
            let operand_u64 = u64::from_str_radix(operand, 10).unwrap();
            operation = match operator {
                "+" => Operation::Add(operand_u64),
                "*" => Operation::Mul(operand_u64),
                _ => unreachable!(),
            };
        }
        // Fourth line: Test: divisble by <test_div>
        let test_div = get_number_from_line(monkey_info_lines.next().unwrap());
        // Fifth line: If true: throw to monkey <true_idx>
        let true_idx = get_number_from_line(monkey_info_lines.next().unwrap()) as usize;
        // Sixth line: If false: throw to monkey <false_idx>
        let false_idx = get_number_from_line(monkey_info_lines.next().unwrap()) as usize;

        monkeys.push(Monkey {
            items: start_items,
            operation: operation,
            test_div: test_div,
            true_idx: true_idx,
            false_idx: false_idx,
            inspections: 0,
        });
    }
    monkeys
}

fn simulate(input: &String, rounds: u32, div3: bool) -> Vec<Monkey> {
    let mut monkeys = get_monkeys(&input);
    let lcm: u64 = monkeys.iter().map(|m| m.test_div).product();
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            while monkeys[monkey_idx].items.len() > 0 {
                let monkey = &mut monkeys[monkey_idx];
                monkey.inspections += 1;
                let old_worry = monkey.items.pop().unwrap();
                let mut new_worry = monkey.update_worry(old_worry);
                if div3 {
                    new_worry /= 3;
                }
                new_worry %= lcm;
                let new_monkey_idx = monkey.test_worry(new_worry);
                drop(monkey);
                monkeys[new_monkey_idx].items.insert(0, new_worry);
            }
        }
    }
    monkeys
}

fn monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    let mut n_inspections: Vec<u64> = monkeys.iter().map(|m| m.inspections).collect();
    n_inspections.sort();
    n_inspections[monkeys.len() - 2..].iter().product()
}

pub fn run() {
    println!("DAY 11:");
    let contents = load_input("data/day11.txt");
    println!(
        "PART 1: {}",
        monkey_business(&simulate(&contents, 20, true))
    );
    println!(
        "PART 2: {}",
        monkey_business(&simulate(&contents, 10000, false))
    );
}

#[test]
fn test_examples() {
    let test_str = load_input("data/test11.txt");
    assert_eq!(monkey_business(&simulate(&test_str, 20, true)), 10605);
    assert_eq!(
        monkey_business(&simulate(&test_str, 10000, false)),
        2713310158
    );
}
