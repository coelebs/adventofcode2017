use util;

static GENERATOR_A_FACTOR: u64 = 16807;
static GENERATOR_B_FACTOR: u64 = 48271;

static GENERATOR_DIVIDER: u64  = 2147483647;

fn generator(input: u64, factor: u64, multiple: u16) -> u64 {
    let mut tmp = input;
    loop {
        tmp = (tmp * factor) % GENERATOR_DIVIDER;

        if tmp % multiple as u64 == 0 {
            break;
        }
    }

    tmp
}

fn find_matching_pairs(start_a: u64, start_b: u64, multiple_a: u16, multiple_b: u16, rounds: usize) -> u32 {
    let mut matching = 0;
    let mut a = generator(start_a, GENERATOR_A_FACTOR, multiple_a);
    let mut b = generator(start_b, GENERATOR_B_FACTOR, multiple_b);

    for _i in 0..rounds {
        if (a & 0xFFFF) == (b & 0xFFFF) {
            matching += 1;
        }

        a = generator(a, GENERATOR_A_FACTOR, multiple_a);
        b = generator(b, GENERATOR_B_FACTOR, multiple_b);
    }

    matching
}

fn solve_part1(start_a: u64, start_b: u64) -> u32 {
    find_matching_pairs(start_a, start_b, 1, 1, 40000000)
}

fn solve_part2(start_a: u64, start_b: u64) -> u32 {
    find_matching_pairs(start_a, start_b, 4, 8, 5000000)
}

pub fn solve_part1_file(path: &str) -> u32 {
    let input: Vec<u64> = util::read_file(&path).ok().unwrap().lines()
        .map(|l| l.split_whitespace().last().unwrap().parse::<u64>().unwrap())
        .collect();

    solve_part1(input[0], input[1])
}

pub fn solve_part2_file(path: &str) -> u32 {
    let input: Vec<u64> = util::read_file(&path).ok().unwrap().lines()
        .map(|l| l.split_whitespace().last().unwrap().parse::<u64>().unwrap())
        .collect();

    solve_part2(input[0], input[1])
}

#[test]
fn test_examples_part1() {
    assert_eq!(solve_part1(65, 8921), 588);
}

#[test]
fn test_examples_part2() {
    assert_eq!(solve_part2(65, 8921), 309);
}

#[test]
fn test_given_input() {
    let input = "inputs/day15.txt";

    assert_eq!(solve_part1_file(input), 619);
    assert_eq!(solve_part2_file(input), 290);
}
