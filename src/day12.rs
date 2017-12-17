use util;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut result = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        if line.len() > 0 {
            let key = iter.next().unwrap().parse::<u32>().unwrap();
            let value = iter.skip(1).map(|c| c.trim_matches(',').parse::<u32>().unwrap()).collect();
            result.insert(key, value);
        }
    }

    result
}

fn count_group(list: &HashMap<u32, Vec<u32>>, group: u32, counted: &mut HashSet<u32>) -> u32 {
    counted.insert(group);

    for item in list[&group].iter() {
        if !counted.contains(item) {
            count_group(list, *item, counted);
        }
        counted.insert(*item);
    }

    counted.len() as u32
}

fn solve_part1(input: &str, group: u32) -> u32 {
    let programs = parse(input);
    let mut counted = HashSet::new();

    count_group(&programs, group, &mut counted)
}

fn solve_part2(input: &str) -> u32 {
    let programs = parse(input);
    let mut counted = HashSet::new();
    let mut groups: u32 = 0;

    for group in programs.keys() {
        if !counted.contains(group) {
            count_group(&programs, *group, &mut counted);
            groups += 1;
        }
    }

    groups
}

pub fn solve_part1_file(path: &str, group: u32) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap(), group)
}

pub fn solve_part2_file(path: &str) -> u32 {
    solve_part2(&util::read_file(path).ok().unwrap())
}

#[test] 
fn test_examples_part1() {
    let input = "
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";

    assert_eq!(solve_part1(input, 0), 6);
}

#[test] 
fn test_examples_part2() {
    let input = "
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";

    assert_eq!(solve_part2(input), 2);
}

#[test]
fn test_given_input() {
    let input = "inputs/day12.txt";

    assert_eq!(solve_part1_file(input, 0), 380);
    assert_eq!(solve_part2_file(input), 181);
}
