use util;
use std::collections::HashMap;

struct Line {
    dest: String,
    operator: Operator,
    val: i32,
    checked: String,
    compare: CompareOperator,
    checked_val: i32,
}

enum Operator {
    Inc,
    Dec,

    Invalid,
}

enum CompareOperator {
    MoreThan,
    LessThan,
    MoreThanEqual,
    LessThanEqual,
    Equal,
    NotEqual,

    Invalid,
}

fn match_operator(input: &str) -> Operator {
    match input {
        "inc" => Operator::Inc,
        "dec" => Operator::Dec,
        _     => Operator::Invalid,
    }
}

fn match_compare_operator(input: &str) -> CompareOperator {
    match input {
        ">"   => CompareOperator::MoreThan,
        "<"   => CompareOperator::LessThan,
        ">="  => CompareOperator::MoreThanEqual,
        "<="  => CompareOperator::LessThanEqual,
        "=="  => CompareOperator::Equal,
        "!="  => CompareOperator::NotEqual,

        _     => CompareOperator::Invalid,
    }
}


fn parse(input: &str) -> Line {
    let mut list = input.split_whitespace();
    let dest = String::from(list.next().unwrap());
    let operator = match_operator(list.next().unwrap());
    let val = list.next().unwrap().parse::<i32>().unwrap();
    let _ignore = list.next();
    let checked = String::from(list.next().unwrap());
    let compare = match_compare_operator(list.next().unwrap());
    let checked_val = list.next().unwrap().parse::<i32>().unwrap();

    Line{dest, operator, val, checked, compare, checked_val}
}

fn run_program(script: &[Line]) -> HashMap<&String, (i32, i32)> {
    let mut reg: HashMap<&String, (i32, i32)> = HashMap::new();
    for line in script {
        let condition;
        {
            let checked = reg.entry(&line.checked).or_insert((0,0));
            condition = match line.compare {
                CompareOperator::MoreThan      => checked.0 >  line.checked_val,
                CompareOperator::LessThan      => checked.0 <  line.checked_val,
                CompareOperator::MoreThanEqual => checked.0 >= line.checked_val,
                CompareOperator::LessThanEqual => checked.0 <= line.checked_val,
                CompareOperator::Equal         => checked.0 == line.checked_val,
                CompareOperator::NotEqual      => checked.0 != line.checked_val,

                CompareOperator::Invalid       => false,
            };
        }

        if condition {
            let dest = reg.entry(&line.dest).or_insert((0,0));
            match line.operator {
                Operator::Inc       => dest.0 += line.val,
                Operator::Dec       => dest.0 -= line.val,

                Operator::Invalid   => eprintln!("Executing with invalid operator"),
            }

            if dest.0 > dest.1 {
                dest.1 = dest.0;
            }
        }
    }

    reg
}

pub fn solve_part1(input: &str) -> i32 {
    let script: Vec<Line> = input.lines().map(|l| parse(l)).collect();
    let reg = run_program(&script);

    reg.values().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0
}

pub fn solve_part1_file(path: &str) -> i32 {
    solve_part1(&util::read_file(path).ok().unwrap())
}

pub fn solve_part2(input: &str) -> i32 {
    let script: Vec<Line> = input.lines().map(|l| parse(l)).collect();
    let reg = run_program(&script);

    reg.values().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1
}

pub fn solve_part2_file(path: &str) -> i32 {
    solve_part2(&util::read_file(path).ok().unwrap())
}

#[test]
pub fn test_examples_part1() {
    let input =
"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    assert_eq!(solve_part1(&input), 1);
}

#[test]
pub fn test_examples_part2() {
    let input =
"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    assert_eq!(solve_part2(&input), 10);
}

#[test]
pub fn test_given_input() {
    let input = "inputs/day8.txt";

    assert_eq!(solve_part1_file(&input), 3880);
    assert_eq!(solve_part2_file(&input), 5035);
}
