#[macro_use]
extern crate serde_derive;
extern crate docopt;

mod day1;
mod day2;
mod day3;
mod day4;

use docopt::Docopt;

const USAGE: &'static str = "
Advent of Code.

Usage:
    advent_of_code day1 <path>
    advent_of_code day2 <path>
    advent_of_code day3 <dest>
    advent_of_code day4 <path>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: String,
    arg_dest: String,
    cmd_day1: bool,
    cmd_day2: bool,
    cmd_day3: bool,
    cmd_day4: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    if args.cmd_day1 {
        let solution1 = day1::solve_part1_file(&args.arg_path).ok().unwrap();
        let solution2 = day1::solve_part2_file(&args.arg_path).ok().unwrap();
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day2 {
        let solution1 = day2::solve_part1_file(&args.arg_path).ok().unwrap();
        let solution2 = day2::solve_part2_file(&args.arg_path).ok().unwrap();
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day3 {
        let solution1 = day3::solve_part1(&args.arg_dest);
        let solution2 = day3::solve_part2(&args.arg_dest);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day4 {
        let solution1 = day4::solve_part1_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2:", solution1);// {:?}", solution1, solution2);
    }
}
