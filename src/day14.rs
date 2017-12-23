use day10;
use std::collections::HashSet;

fn parse_hex(input: &str) -> Vec<u32> {
    let mut result = vec![];
    let mut tmp = input.chars().peekable();

    while tmp.peek().is_some() {
        let hex: String = tmp.by_ref().take(2).collect();
        result.push(u32::from_str_radix(&hex, 16).unwrap() as u32);
    }

    result
}

fn generate_grid(key: &str) -> Vec< Vec<u32> > {
    let mut result = vec![];
    for i in 0..128 {
        result.push(parse_hex(&day10::solve_part2(&format!("{}-{}", key, i), 256)));
    }
    result
}

fn count_used(grid: &Vec< Vec<u32> >) -> u32 {
    let mut count = 0;
    for row in grid {
        for item in row {
            count += item.count_ones();
        }
    }

    count
}

fn find_group(grid: &Vec<Vec<u32>>, seen: &mut HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    let bit = col % 8;
    let q = col / 8;

    let val = grid[row][q];

    if (((val << bit) & 0b10000000) >> 7) == 0 || seen.contains(&(row,col)) {
        return false;
    }

    seen.insert((row,col));
    
    if row > 0 {
        find_group(grid, seen, row - 1, col); 
    }
    if col > 0 {
        find_group(grid, seen, row, col - 1); 
    }
    if row < (grid.len() - 1) {
        find_group(grid, seen, row + 1, col); 
    }
    if col < ((grid[row].len() * 8) - 1) {
        find_group(grid, seen, row, col + 1); 
    }

    true
}

fn detect_group(grid: &Vec<Vec<u32>>) -> u32 {
    let mut seen: HashSet<(usize, usize)> = HashSet::new(); 
    let mut result = 0;

    for row in 0..grid.len() {
        for col_step in 0..grid[row].len() {
            for bit in 0..8 {
                let col = (col_step * 8) + bit;

                if find_group(grid, &mut seen, row, col) {
                    result += 1;
                }
            }
        }
    }
    
    result
}

pub fn solve_part1(input: &str) -> u32 {
    let grid = generate_grid(input);
    count_used(&grid)
}

pub fn solve_part2(input: &str) -> u32 {
    let mut group_grid = generate_grid(input);

    detect_group(&mut group_grid)
}

#[test]
fn test_examples_part1() {
    let input = "flqrgnkx";
    
    assert_eq!(solve_part1(input), 8108);
}

#[test]
fn test_examples_part2() {
    let input = "flqrgnkx";
    
    assert_eq!(solve_part2(input), 1242);
}

#[test]
fn test_given_input() {
    let input = "jxqlasbh";

    assert_eq!(solve_part1(input), 8140);
    assert_eq!(solve_part2(input), 1182);
}
