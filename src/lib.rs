#![feature(hash_drain_filter)] // Allows me to use HashMap::drain_filter on day 18
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day18;

use regex::Regex;
use std::fs;
use std::io::Read;

pub fn load_input(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
