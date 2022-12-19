use crate::load_input;
use std::collections::HashMap;

fn simulate(input: &String, n_knots: u32) -> u32 {
    let mut knots: Vec<[i32; 2]> = Vec::new();
    let last_idx = (n_knots - 1) as usize;
    for _ in 0..n_knots {
        knots.push([0, 0]);
    }
    let mut visited: HashMap<[i32; 2], u32> = HashMap::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        chars.next().unwrap();
        let steps = u32::from_str_radix(&chars.collect::<String>()[..], 10).unwrap();
        let move_head: Box<dyn Fn(&mut Vec<[i32; 2]>) -> ()> = match direction {
            'L' => Box::new(|hd| hd[0][0] -= 1),
            'R' => Box::new(|hd| hd[0][0] += 1),
            'D' => Box::new(|hd| hd[0][1] -= 1),
            'U' => Box::new(|hd| hd[0][1] += 1),
            _ => panic!("Uh Oh!"),
        };

        for _ in 0..steps {
            // Move head
            move_head(&mut knots);
            move_rest(&mut knots);
            if visited.contains_key(&knots[last_idx]) {
                *visited.get_mut(&knots[last_idx]).unwrap() += 1;
            } else {
                visited.insert(knots[last_idx].clone(), 1);
            }
        }
    }

    visited.len() as u32
}

fn move_rest(knots: &mut Vec<[i32; 2]>) {
    for i in 0..knots.len() - 1 {
        let hdiff = knots[i][0] - knots[i + 1][0];
        let vdiff = knots[i][1] - knots[i + 1][1];
        if hdiff.abs() < 2 && vdiff.abs() < 2 {
            return;
        } else if hdiff.abs() == 2 {
            knots[i + 1][0] += 1 * hdiff.signum();
            if vdiff != 0 {
                knots[i + 1][1] += 1 * vdiff.signum();
            }
        } else if vdiff.abs() == 2 {
            knots[i + 1][1] += 1 * vdiff.signum();
            if hdiff != 0 {
                knots[i + 1][0] += 1 * hdiff.signum();
            }
        }
    }
}

fn part1(input: &String) -> u32 {
    simulate(input, 2)
}

fn part2(input: &String) -> u32 {
    simulate(input, 10)
}

pub fn run() {
    println!("DAY 9:");
    let contents = load_input("data/day09.txt");
    println!("PART 1: {}", part1(&contents));
    println!("PART 2: {}", part2(&contents));
}

#[test]
fn test_examples() {
    let test_str_1 = String::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n");
    assert_eq!(part1(&test_str_1), 13);
    let test_str_2 = String::from("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n");
    assert_eq!(part2(&test_str_2), 36);
}
