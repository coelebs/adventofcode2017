#[macro_use]
extern crate serde_derive;
extern crate docopt;

mod day1;

use docopt::Docopt;

const USAGE: &'static str = "
Advent of Code.

Usage:
    advent_of_code day1 <captcha>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_captcha: String,
    cmd_day1: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    if args.cmd_day1 {
        let solution1 = day1::solve_part1(&args.arg_captcha);
        let solution2 = day1::solve_part2(&args.arg_captcha);
        println!("Solution part 1: {:?}\nSolution part 2: {:?}", solution1, solution2);
    }
}
