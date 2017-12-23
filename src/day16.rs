use util;

static BILLION: usize = 1000000000;

fn spin(input: &mut Vec<char>, size: usize) { 
    let tmp = input.to_vec();

    for i in 0..input.len() {
        input[(i + size) % tmp.len()] = tmp[i];
    }
}

fn exchange(input: &mut Vec<char>, a: usize, b: usize) {
    input.swap(a, b);
}

fn partner(input: &mut Vec<char>, a: char, b: char) {
    let a_pos = input.iter().position(|c| *c == a).unwrap();
    let b_pos = input.iter().position(|c| *c == b).unwrap();
    input.swap(a_pos, b_pos);
}

fn solve_part1(input: &str, routine: &str) -> String {
    let mut result = input.chars().collect();

    for step in routine.split(',') {
        let dance = step.chars().nth(0).unwrap();
        let args: Vec<&str> = step[1..step.len()].split('/').collect();
        match dance {
            's' => spin(&mut result, args[0].parse::<usize>().unwrap()),
            'x' => exchange(&mut result, args[0].parse::<usize>().unwrap(), 
                                         args[1].parse::<usize>().unwrap()),
            'p' => partner(&mut result, args[0].chars().nth(0).unwrap(), args[1].chars().nth(0).unwrap()),
            _   => panic!("Unknown dance move!"),
        }
    }

    result.iter().collect::<String>()
}

fn solve_part2(input: &str, routine: &str) -> String {
    let mut history = vec![solve_part1(input, routine)];
    let prefix;

    loop {
        let new = solve_part1(history.last().unwrap(), routine);

        let pos = history.iter().position(|s| *s == new);

        history.push(new);

        if pos.is_some() {
            prefix = pos.unwrap(); 
            break;
        }
    }

    let postfix = (BILLION - prefix) % (history.len() - prefix);

    for _i in 0..(postfix + 1) {
        let new = solve_part1(history.last().unwrap(), routine);
        history.push(new);
    }

    history.pop().unwrap()
}

pub fn solve_part1_file(path: &str) -> String {
    solve_part1("abcdefghijklmnop", util::read_file(path).ok().unwrap().trim()) 
}

pub fn solve_part2_file(path: &str) -> String {
    solve_part2("abcdefghijklmnop", util::read_file(path).ok().unwrap().trim()) 
}

#[test]
fn test_examples_part1() {
    let input = "abcde";
    let routine = "s1,x3/4,pe/b";

    assert_eq!(solve_part1(input, routine), "baedc");
}

#[test]
fn test_examples_part2() {
    let routine = "s1,x3/4,pe/b";
    let input = solve_part1("abcde", routine);

    assert_eq!(solve_part1(&input, routine), "ceadb");
}

#[test]
fn test_given_input() {
    let path = "inputs/day16.txt";

    assert_eq!(solve_part1_file(path), "fnloekigdmpajchb");
    assert_eq!(solve_part2_file(path), "amkjepdhifolgncb");
}
