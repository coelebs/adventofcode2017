use util;

fn find_duplicate_word(input: Vec<String>) -> bool {
   let mut set = input.to_vec(); 
   set.sort();
   set.dedup();
   
   set.len() == input.len()
}

pub fn solve_part1(input: &str) -> bool {
    let mut key: Vec<String> = vec![];

    for word in input.split_whitespace() {
        key.push(String::from(word));
    }

    find_duplicate_word(key) 
}

pub fn solve_part2(input: &str) -> bool {
    let mut key: Vec<String> = vec![];

    for word in input.split_whitespace() {
        let mut word_vec = vec![];
        for char in word.chars() {
            word_vec.push(char);
        }
        word_vec.sort();
        
        key.push(word_vec.into_iter().collect());
    }

    find_duplicate_word(key) 
}

pub fn solve_part1_file(path: &str) -> u32 {
    let contents = util::read_file(path).ok().unwrap();

    let mut solution = 0;

    for line in contents.lines() {
        if solve_part1(line) {
            solution += 1;
        }
    }

    solution
}

pub fn solve_part2_file(path: &str) -> u32 {
    let contents = util::read_file(path).ok().unwrap();

    let mut solution = 0;

    for line in contents.lines() {
        if solve_part2(line) {
            solution += 1;
        }
    }

    solution
}

#[test]
fn test_given_input() {
    let input = "inputs/day4.txt";

    assert_eq!(solve_part1_file(input), 455);
    assert_eq!(solve_part2_file(input), 186);
}

#[test]
fn test_example_part1() {
    let input = ["aa bb cc dd ee", "aa bb cc dd aa", "aa bb cc dd aaa"];
    let valid = [true,             false,            true];

    for (inp, outp) in input.iter().zip(valid.iter()) {
        assert_eq!(solve_part1(inp), *outp);
    }
}

#[test]
fn test_example_part2() {
    let input = ["abcde fghij", "abcde xyz ecdab", "a ab abc abd abf abj", "iiii oiii ooii oooi oooo", "oiii ioii iioi iiio"];
    let valid = [true, false, true, true, false];

    for (inp, outp) in input.iter().zip(valid.iter()) {
        assert_eq!(solve_part2(inp), *outp);
    }
}
