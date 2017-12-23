use util;

static GENERATOR_A_FACTOR: u64 = 16807;
static GENERATOR_B_FACTOR: u64 = 48271;

static GENERATOR_DIVIDER: u64  = 2147483647;

static ROUNDS: usize = 40000000;

fn generator(input: u64, factor: u64) -> u64 {
    (input * factor) % GENERATOR_DIVIDER
}

fn solve_part1(start_a: u64, start_b: u64, rounds: usize) -> u32 {
    let mut matching = 0;
    let mut a = generator(start_a, GENERATOR_A_FACTOR);
    let mut b = generator(start_b, GENERATOR_B_FACTOR);

    for _i in 0..rounds {
        if (a & 0xFFFF) == (b & 0xFFFF) {
            matching += 1;
        }

        a = generator(a, GENERATOR_A_FACTOR);
        b = generator(b, GENERATOR_B_FACTOR);
    }

    matching
}

pub fn solve_part1_file(path: &str) -> u32 {
    let input: Vec<u64> = util::read_file(&path).ok().unwrap().lines()
        .map(|l| l.split_whitespace().last().unwrap().parse::<u64>().unwrap())
        .collect();

    solve_part1(input[0], input[1], ROUNDS)
}

#[test]
fn test_examples_part1() {
    assert_eq!(solve_part1(65, 8921, ROUNDS), 588);
}

#[test]
fn test_given_input() {
    let input = "inputs/day15.txt";

    assert_eq!(solve_part1_file(input), 619);
}
