use util;

fn parse_input(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn solve_first_captcha(input: &[u32]) -> u32 {
    let mut total: u32 = 0;
    let mut prev: &u32 = input.last().unwrap();

    for num in input {
        if num == prev {
            total += *num;
        }

        prev = num;
    }

    total
}

fn solve_second_captcha(input: &[u32]) -> u32 {
    let jump = input.len()/2;
    let mut total: u32 = 0;

    for i in 0..input.len() {
        if input[i] == input[(i + jump) % input.len()] {
            total += input[i];
        }
    }

    total
}

fn solve_part1(input: &str) -> u32 {
    let nums = parse_input(input);
    solve_first_captcha(&nums)
}

fn solve_part2(input: &str) -> u32 {
    let nums = parse_input(input);
    solve_second_captcha(&nums)
}

pub fn solve_part1_file(path: &str) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap().trim())
}

pub fn solve_part2_file(path: &str) -> u32 {
    solve_part2(&util::read_file(path).ok().unwrap().trim())
}

#[test]
fn test_examples_part1() {
    let inputs      = ["1122",  "1111",     "1234",     "91212129"];
    let solutions   = [3,       4,          0,          9]; 

    for i in 0..inputs.len() {
        assert_eq!(solve_part1(inputs[i]), solutions[i]);
    }
}

#[test]
fn test_examples_part2() {
    let inputs      = ["1212",  "1221",     "123425",   "123123",   "12131415"];
    let solutions   = [6,       0,          4,          12,         4];

    for i in 0..inputs.len() {
        assert_eq!(solve_part2(inputs[i]), solutions[i]);
    }
}

#[test]
fn test_given_input() {
    let input = "inputs/day1.txt";

    assert_eq!(solve_part1_file(input), 1069);
    assert_eq!(solve_part2_file(input), 1268);
}
