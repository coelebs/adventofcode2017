use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn jump_maze(mut maze: Vec<i32>, advanced: bool) -> u32 {
    let mut jumps = 0; 
    let mut i: i32 = 0;

    while i < maze.len() as i32 && i >= 0 {
        let curpos = i as usize;
        i += maze[curpos];

        if advanced && maze[curpos] >= 3 {
            maze[curpos] -= 1;
        } else {
            maze[curpos] += 1;
        }

        jumps += 1;
    }

    jumps
}

pub fn solve(input: &str, advance: bool) -> u32 {
    jump_maze(input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect(), advance)
}

pub fn solve_part1_file(path: &str) -> Result<u32, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(solve(&contents, false))
}

pub fn solve_part2_file(path: &str) -> Result<u32, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(solve(&contents, true))
}

#[test]
fn test_examples() {
    let input = "0 3 0 1 -3";

    assert_eq!(solve(input, false), 5);
    assert_eq!(solve(input, true), 10);
}

#[test]
fn test_given_input() {
    let input = "inputs/day5.txt";

    assert_eq!(solve_part1_file(input).ok(), Some(358309));
    assert_eq!(solve_part2_file(input).ok(), Some(28178177));
}
