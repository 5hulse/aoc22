use crate::load_input;
use ndarray::Array2;
use std::collections::HashMap;

fn make_tree_array(input: &String) -> Array2<u32> {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().chars().count();
    let mut data = Vec::<u32>::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        data.extend_from_slice(&row);
    }
    Array2::<u32>::from_shape_vec((nrows, ncols), data).unwrap()
}

fn tree_check(can_see: &mut HashMap<u32, u32>, tree: u32, mut max_so_far: u32, idx: u32) -> u32 {
    if tree > max_so_far {
        max_so_far = tree;
        if !can_see.contains_key(&idx) {
            can_see.insert(idx, tree);
        }
    }
    max_so_far
}

fn part1(input: &String) -> u32 {
    let array = make_tree_array(&input);
    let shape = array.shape();
    let nrows = shape[0];
    let ncols = shape[1];

    let mut can_see = HashMap::<u32, u32>::new();

    // LOOP OVER TOP and BOTTOM ROWS
    for i in 0..ncols {
        can_see.insert(i as u32, array[[0, i]]);
        can_see.insert((nrows * (ncols - 1) + i) as u32, array[[nrows - 1, i]]);
    }

    // LOOP OVER LEFTMOST AND RIGHTMOST COLUMNS
    for i in 1..nrows - 1 {
        can_see.insert((i * ncols) as u32, array[[i, 0]]);
        can_see.insert((i * ncols + (ncols - 1)) as u32, array[[i, ncols - 1]]);
    }

    // LOOKING FROM LEFT AND FROM RIGHT
    for (i, row) in array.rows().into_iter().enumerate() {
        // LEFT
        let mut max_so_far = row[0];
        for (j, tree) in row.iter().enumerate() {
            let idx = (i * ncols + j) as u32;
            max_so_far = tree_check(&mut can_see, *tree, max_so_far, idx);
            if max_so_far == 9 {
                break;
            }
        }
        // RIGHT
        let mut max_so_far = row[ncols - 1];
        for (j, tree) in row.iter().rev().enumerate() {
            let idx = (i * ncols + (ncols - j - 1)) as u32;
            max_so_far = tree_check(&mut can_see, *tree, max_so_far, idx);
            if max_so_far == 9 {
                break;
            }
        }
    }

    // LOOKING FROM TOP AND BOTTOM
    for (i, col) in array.columns().into_iter().enumerate() {
        // TOP
        let mut max_so_far = col[0];
        for (j, tree) in col.iter().enumerate() {
            let idx = (i + j * ncols) as u32;
            max_so_far = tree_check(&mut can_see, *tree, max_so_far, idx);
            if max_so_far == 9 {
                break;
            }
        }
        // BOTTOM
        let mut max_so_far = col[nrows - 1];
        for (j, tree) in col.iter().rev().enumerate() {
            let idx = (((nrows - j - 1) * ncols) + i) as u32;
            max_so_far = tree_check(&mut can_see, *tree, max_so_far, idx);
            if max_so_far == 9 {
                break;
            }
        }
    }

    can_see.len() as u32
}

fn part2(input: &String) -> u32 {
    let array = make_tree_array(&input);
    let shape = array.shape();
    let nrows = shape[0];
    let ncols = shape[1];

    let mut max_scenic_score: u32 = 0;
    for row in 0..nrows {
        for col in 0..ncols {
            let proposoed_tree = array[[row, col]];

            // LOOKING LEFT
            let mut idx = col;
            let mut l_scenic_score: u32 = 0;
            loop {
                if idx == 0 {
                    break;
                }
                l_scenic_score += 1;
                idx -= 1;
                if array[[row, idx]] >= proposoed_tree {
                    break;
                }
            }

            // LOOKING RIGHT
            let mut idx = col;
            let mut r_scenic_score: u32 = 0;
            loop {
                if idx == ncols - 1 {
                    break;
                }
                r_scenic_score += 1;
                idx += 1;
                if array[[row, idx]] >= proposoed_tree {
                    break;
                }
            }

            // LOOKING UP
            let mut idx = row;
            let mut u_scenic_score: u32 = 0;
            loop {
                if idx == 0 {
                    break;
                }
                u_scenic_score += 1;
                idx -= 1;
                if array[[idx, col]] >= proposoed_tree {
                    break;
                }
            }

            // LOOKING DOWN
            let mut idx = row;
            let mut d_scenic_score: u32 = 0;
            loop {
                if idx == nrows - 1 {
                    break;
                }
                d_scenic_score += 1;
                idx += 1;
                if array[[idx, col]] >= proposoed_tree {
                    break;
                }
            }

            let scenic_score = l_scenic_score * r_scenic_score * u_scenic_score * d_scenic_score;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    max_scenic_score
}

pub fn run() {
    println!("DAY 8:");
    let contents = load_input("data/day08.txt");
    println!("PART 1: {}", part1(&contents));
    println!("PART 2: {}", part2(&contents));
}

#[test]
fn test_examples() {
    let test_str = String::from("30373\n25512\n65332\n33549\n35390\n");
    assert_eq!(part1(&test_str), 21);
    assert_eq!(part2(&test_str), 8);
}
