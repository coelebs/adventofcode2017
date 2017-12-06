use util;

fn balance_blocks(mem: &mut Vec<u32>) {
    let mut high = 0;
    let mut index = 0;

    for (i, value) in mem.iter().enumerate() {
        if *value > high {
            high = *value;
            index = i;
        }
    }

    mem[index] = 0;

    while high > 0 {
        index = (index + 1) % mem.len();
        mem[index] += 1;
        high -= 1;
    }
}

fn find_repeating_balance(mem: &mut Vec<u32>) -> (u32, u32) {
    let mut history: Vec< Vec<u32> > = vec![];
    let mut iterations = 0;

    while history.iter().find(|x| *x == mem).is_none() {
        history.push(mem.to_vec());
        balance_blocks(mem);

        iterations += 1;
    }

    (iterations, iterations - history.iter().position(|x| x == mem).unwrap() as u32)
}

pub fn solve_part1(input: &str) -> u32 {
    find_repeating_balance(&mut input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect()).0
}

pub fn solve_part1_file(path: &str) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap())
}

pub fn solve_part2(input: &str) -> u32 {
    find_repeating_balance(&mut input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect()).1
}

pub fn solve_part2_file(path: &str) -> u32 {
    solve_part2(&util::read_file(path).ok().unwrap())
}

#[test]
fn test_examples_part1() { 
    let input = "0 2 7 0";

    assert_eq!(solve_part1(input), 5);
}

#[test]
fn test_examples_part2() { 
    let input = "0 2 7 0";

    assert_eq!(solve_part2(input), 4);
}

#[test] 
fn test_given_input() {
    let input = "inputs/day6.txt";

    assert_eq!(solve_part1_file(input), 4074);
    assert_eq!(solve_part2_file(input), 2793);
}
