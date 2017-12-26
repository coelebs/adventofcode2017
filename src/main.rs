#[macro_use]
extern crate serde_derive;
extern crate docopt;

mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

use docopt::Docopt;

const USAGE: &'static str = "
Advent of Code.

Usage:
    advent_of_code day1 <path>
    advent_of_code day2 <path>
    advent_of_code day3 <dest>
    advent_of_code day4 <path>
    advent_of_code day5 <path>
    advent_of_code day6 <path>
    advent_of_code day7 <path>
    advent_of_code day8 <path>
    advent_of_code day9 <path>
    advent_of_code day10 <path>
    advent_of_code day11 <path>
    advent_of_code day12 <path> <group>
    advent_of_code day13 <path>
    advent_of_code day14 <key>
    advent_of_code day15 <path>
    advent_of_code day16 <path>
    advent_of_code day17 <input>
    advent_of_code day18 <path>
    advent_of_code day19 <path>
    advent_of_code day20 <path>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: String,
    arg_dest: String,
    arg_group: u32,
    arg_input: u32,
    arg_key: String,
    cmd_day1: bool,
    cmd_day2: bool,
    cmd_day3: bool,
    cmd_day4: bool,
    cmd_day5: bool,
    cmd_day6: bool,
    cmd_day7: bool,
    cmd_day8: bool,
    cmd_day9: bool,
    cmd_day10: bool,
    cmd_day11: bool,
    cmd_day12: bool,
    cmd_day13: bool,
    cmd_day14: bool,
    cmd_day15: bool,
    cmd_day16: bool,
    cmd_day17: bool,
    cmd_day18: bool,
    cmd_day19: bool,
    cmd_day20: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE) .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    if args.cmd_day1 {
        let solution1 = day1::solve_part1_file(&args.arg_path);
        let solution2 = day1::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day2 {
        let solution1 = day2::solve_part1_file(&args.arg_path);
        let solution2 = day2::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day3 {
        let solution1 = day3::solve_part1(&args.arg_dest);
        let solution2 = day3::solve_part2(&args.arg_dest);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day4 {
        let solution1 = day4::solve_part1_file(&args.arg_path);
        let solution2 = day4::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day5 {
        let solution1 = day5::solve_part1_file(&args.arg_path);
        let solution2 = day5::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day6 {
        let solution1 = day6::solve_part1_file(&args.arg_path);
        let solution2 = day6::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day7 {
        let solution1 = day7::solve_part1_file(&args.arg_path);
        let solution2 = day7::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1.name, solution2);
    }

    if args.cmd_day8 {
        let solution1 = day8::solve_part1_file(&args.arg_path);
        let solution2 = day8::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day9 {
        let solution1 = day9::solve_part1_file(&args.arg_path);
        let solution2 = day9::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day10 {
        let solution1 = day10::solve_part1_file(&args.arg_path);
        let solution2 = day10::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day11 {
        let solution1 = day11::solve_part1_file(&args.arg_path);
        let solution2 = day11::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day12 {
        let solution1 = day12::solve_part1_file(&args.arg_path, args.arg_group);
        let solution2 = day12::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day13 {
        let solution1 = day13::solve_part1_file(&args.arg_path);
        let solution2 = day13::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day14 {
        let solution1 = day14::solve_part1(&args.arg_key);
        let solution2 = day14::solve_part2(&args.arg_key);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day15 {
        let solution1 = day15::solve_part1_file(&args.arg_path);
        let solution2 = day15::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day16 {
        let solution1 = day16::solve_part1_file(&args.arg_path);
        let solution2 = day16::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day17 {
        let solution1 = day17::solve_part1(args.arg_input);
        let solution2 = day17::solve_part2(args.arg_input);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day18 {
        let solution1 = day18::solve_part1_file(&args.arg_path);
        let solution2 = day18::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day19 {
        let solution1 = day19::solve_part1_file(&args.arg_path);
        let solution2 = day19::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }

    if args.cmd_day20 {
        let solution1 = day20::solve_part1_file(&args.arg_path);
        let solution2 = 0;//day20::solve_part2_file(&args.arg_path);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }
}
