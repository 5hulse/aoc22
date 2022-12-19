use crate::load_input;

fn part1(input: &String) -> i32 {
    let mut ncycles: u32 = 0;
    let mut x: i32 = 1;
    let mut signal_strength_sum: i32 = 0;
    for line in input.lines() {
        if line == "noop" {
            ncycles += 1;
            if cycle_of_interest(ncycles) {
                signal_strength_sum += (ncycles as i32) * x;
            }
        } else if &line[..4] == "addx" {
            for _ in 0..2 {
                ncycles += 1;
                if cycle_of_interest(ncycles) {
                    signal_strength_sum += (ncycles as i32) * x;
                }
            }
            let to_add = i32::from_str_radix(line.split(" ").nth(1).unwrap(), 10).unwrap();
            x += to_add;
        }
    }
    signal_strength_sum
}

fn part2(input: &String) -> String {
    let mut ncycles: u32 = 0;
    let mut x: i32 = 1;
    let mut display = String::new();
    for line in input.lines() {
        if line == "noop" {
            add_pixel(&mut display, ncycles, x);
            ncycles += 1;
        } else {
            for _ in 0..2 {
                add_pixel(&mut display, ncycles, x);
                ncycles += 1;
            }
            let to_add = i32::from_str_radix(line.split(" ").nth(1).unwrap(), 10).unwrap();
            x += to_add;
        }
    }
    display.pop(); // Remove last newline
    display
}

fn get_position(ncycles: u32) -> u32 {
    ncycles % 40
}

fn add_pixel(display: &mut String, ncycles: u32, x: i32) {
    let position = get_position(ncycles);
    if ((x - 1) as u32 <= position) && (position <= (x + 1) as u32) {
        display.push('#');
    } else {
        display.push('.');
    }
    if position == 39 {
        display.push('\n');
    }
}

fn cycle_of_interest(cycle: u32) -> bool {
    if cycle < 20 {
        false
    } else if (cycle - 20) % 40 == 0 {
        true
    } else {
        false
    }
}

pub fn run() {
    println!("DAY 10:");
    let contents = load_input("data/day10.txt");
    println!("PART 1: {}", part1(&contents));
    println!("PART 2:\n{}", part2(&contents));
}

#[test]
fn test_examples() {
    let test_str = load_input("data/test10.txt");
    assert_eq!(part1(&test_str), 13140);
    assert_eq!(
        part2(&test_str),
        String::from(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        ),
    );
}
