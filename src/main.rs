use structopt::StructOpt;

use aoc22;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    day: usize,
}

fn main() {
    let opt = Opt::from_args();
    match opt.day {
        1 => aoc22::day01::run(),
        2 => aoc22::day02::run(),
        3 => aoc22::day03::run(),
        4 => aoc22::day04::run(),
        5 => aoc22::day05::run(),
        6 => aoc22::day06::run(),
        _ => println!("Unregonised day argument"),
    };
}
