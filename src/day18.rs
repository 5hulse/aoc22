use crate::load_input;
use std::collections::HashMap;
use std::convert::TryInto;

fn get_coordinates(input: &String) -> Vec<[i32; 3]> {
    let mut coordinates: Vec<[i32; 3]> = Vec::new();
    for line in input.lines() {
        coordinates.push(
            line.split(",")
                .map(|s| i32::from_str_radix(s, 10).unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap(),
        );
    }
    coordinates
}

fn faces_touch(cube1: &[i32; 3], cube2: &[i32; 3]) -> bool {
    let mut n_equal: i32 = 0;
    for (c1, c2) in cube1.iter().zip(cube2.iter()) {
        if (c1 - c2).abs() > 1 {
            return false;
        } else if c1 == c2 {
            n_equal += 1
        }
    }
    if n_equal == 2 {
        true
    } else {
        false
    }
}

fn find_adjacent_coordinates(coordiante: &[i32; 3]) -> Vec<[i32; 3]> {
    let mut adjacent_coords: Vec<[i32; 3]> = Vec::new();
    for i in 0..3 {
        let mut clone1 = coordiante.clone();
        clone1[i] += 1;
        adjacent_coords.push(clone1);

        let mut clone2 = coordiante.clone();
        clone2[i] -= 1;
        adjacent_coords.push(clone2);
    }
    adjacent_coords
}

fn part1(coordinates: &Vec<[i32; 3]>) -> i32 {
    let mut n_connections = 0;
    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            if faces_touch(&coordinates[i], &coordinates[j]) {
                n_connections += 2;
            }
        }
    }
    (6 * coordinates.len() as i32) - n_connections
}

fn part2(coordinates: &Vec<[i32; 3]>, total_surface_area: i32) -> i32 {
    let mut adjacent_coords: HashMap<[i32; 3], i32> = HashMap::new();
    for coordiante in coordinates {
        for adjacent_coord in find_adjacent_coordinates(&coordiante) {
            match adjacent_coords.get_mut(&adjacent_coord) {
                Some(ac) => *ac += 1,
                None => {
                    adjacent_coords.insert(adjacent_coord, 1);
                    ()
                }
            }
        }
    }
    // Get all coordinates with 6 surrounding cubes that do not exist
    // in the coordinate map
    let air_pockets: Vec<[i32; 3]> = adjacent_coords
        .drain_filter(|k, v| *v == 6 && !coordinates.contains(&k))
        .collect::<HashMap<[i32; 3], i32>>()
        .into_keys()
        .collect();
    total_surface_area - part1(&air_pockets)
}

pub fn run() {
    println!("DAY 18:");
    let contents = load_input("data/day18.txt");
    let coordinates = get_coordinates(&contents);
    let total_surface_area = part1(&coordinates);
    println!("PART 1: {}", total_surface_area);
    println!("PART 2: {}", part2(&coordinates, total_surface_area));
}

#[test]
fn test_examples() {
    let test_str = String::from(
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
    );
    let coordinates = get_coordinates(&test_str);
    let total_surface_area = part1(&coordinates);
    assert_eq!(total_surface_area, 64);
    assert_eq!(part2(&coordinates, total_surface_area), 58);
}
