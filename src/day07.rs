use crate::load_input;
use std::collections::HashMap;

pub fn run() {
    println!("DAY 7:");
    let contents = load_input("data/day07.txt");
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut current_dir = String::from("/");
    let mut lines = contents.as_str().lines();
    let mut line = lines.next().unwrap();
    loop {
        if &line[..4] == "$ cd" {
            let dir = &line[..].split(" ").nth(2).unwrap();
            if dir == &".." {
                current_dir.pop().unwrap();
                current_dir.truncate(current_dir.rfind('/').unwrap() + 1);
            } else if dir == &"/" {
                current_dir.truncate(1);
            } else {
                current_dir.push_str(dir);
                current_dir.push('/');
            }
            line = lines.next().unwrap();
        } else if &line[..4] == "$ ls" {
            let eof: bool;
            loop {
                // Get next line, and check haven't reach end of iterator
                if let Some(l) = lines.next() {
                    line = l;
                } else {
                    eof = true;
                    break;
                }

                let c0 = line.chars().next().unwrap();
                if c0 == '$' {
                    eof = false;
                    break;
                }

                if c0.is_ascii_digit() {
                    let first_word = &line[0..line.find(' ').unwrap()];
                    let size = first_word.parse::<u32>().unwrap();
                    for idx in current_dir.rmatch_indices("/").map(|t| t.0) {
                        let mut key = current_dir.clone();
                        key.truncate(idx + 1);
                        if dirs.contains_key(&key) {
                            *dirs.get_mut(&key).unwrap() += size;
                        } else {
                            dirs.insert(key.clone(), size);
                        }
                    }
                }
            }
            if eof {
                break;
            }
        }
    }

    let mut total1: u32 = 0;
    for dir_size in dirs.values() {
        if *dir_size <= 100000 {
            total1 += dir_size;
        }
    }
    println!("TASK 1: {}", total1);

    let free_space: u32 = 70000000 - dirs.get("/").unwrap();

    let space_needed: u32 = 30000000 - free_space;
    let mut space_to_free = None;
    for dir_size in dirs.values() {
        if *dir_size >= space_needed {
            if let Some(curr) = space_to_free {
                if dir_size < curr {
                    space_to_free = Some(dir_size)
                }
            } else {
                space_to_free = Some(dir_size)
            }
        }
    }
    println!("TASK 2: {}", space_to_free.unwrap());
}
