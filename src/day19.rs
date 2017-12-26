use util;

#[derive(PartialEq)]
enum Direction {
    North,
    East, 
    South, 
    West,
}

fn make_matrix(input: &str) -> Vec< Vec<char>> {
    input.lines()
         .map(|l| l.chars().collect())
         .collect()
}

fn find_next(grid: &Vec< Vec<char>>, row: usize, col: usize, dir: &mut Direction) -> Option<(usize, usize)> {
    let result = match *dir {
        Direction::North  => if row > 0                 && !grid[row-1][col].is_whitespace() { Some((row - 1, col)) } else { None },
        Direction::East   => if col < grid[row].len()-1 && !grid[row][col+1].is_whitespace() { Some((row, col + 1)) } else { None },
        Direction::South  => if row < grid.len()-1      && !grid[row+1][col].is_whitespace() { Some((row + 1, col)) } else { None },
        Direction::West   => if col > 0                 && !grid[row][col-1].is_whitespace() { Some((row, col - 1)) } else { None },
    };

    if result.is_some() {
        result
    } else if *dir != Direction::South && row > 0 && !grid[row-1][col].is_whitespace() {
        *dir = Direction::North;
        Some((row-1,col))
    } else if *dir != Direction::West && col < grid[row].len() - 1 && !grid[row][col+1].is_whitespace() {
        *dir = Direction::East;
        Some((row,col+1))
    } else if *dir != Direction::North && row < grid.len() - 1 && !grid[row+1][col].is_whitespace() {
        *dir = Direction::South;
        Some((row+1,col))
    } else if *dir != Direction::East && col > 0 && !grid[row][col-1].is_whitespace() {
        *dir = Direction::West;
        Some((row,col-1))
    } else {
        None
    }
}

fn follow_line(grid: &Vec< Vec<char>>, col: usize) -> (String, u32) {
    let mut result = String::new();
    let mut dir = Direction::South;
    let mut next = find_next(&grid, 0, col, &mut dir);
    let mut steps = 1;
    
    while next.is_some() {
        let (row, col) = next.unwrap();
        if grid[row][col].is_alphanumeric() {
            result.push(grid[row][col]);
        }

        next = find_next(&grid, row, col, &mut dir);
        steps += 1;
    }

    (result, steps)
}

fn solve(input: &str) -> (String, u32) {
    let matrix = make_matrix(input);
    let mut col = 0;
    for i in 0..matrix[0].len() {
        if !matrix[0][i].is_whitespace() {
            col = i;
        }
    }

    follow_line(&matrix, col)
}

fn solve_part1(input: &str) -> String {
    solve(input).0
}

fn solve_part2(input: &str) -> u32 {
    solve(input).1
}

pub fn solve_part1_file(path: &str) -> String {
    solve_part1(&util::read_file(path).ok().unwrap())
}

pub fn solve_part2_file(path: &str) -> u32 {
    solve_part2(&util::read_file(path).ok().unwrap())
}

#[test]
fn test_examples_part1() { 
    let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+"; 
        
     assert_eq!(solve_part1(input), "ABCDEF");
}

#[test]
fn test_examples_part2() { 
    let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+"; 
        
     assert_eq!(solve_part2(input), 38);
}

#[test]
fn test_given_input() {
    let input = "inputs/day19.txt";

    assert_eq!(solve_part1_file(input), "BPDKCZWHGT");
}
