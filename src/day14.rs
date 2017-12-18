use day10;

fn parse_hex(input: &str) -> Vec<i32> {
    let mut result = vec![];
    let mut tmp = input.chars().peekable();

    while tmp.peek().is_some() {
        let hex: String = tmp.by_ref().take(2).collect();
        result.push(i32::from_str_radix(&hex, 16).unwrap());
    }

    result
}

fn generate_grid(key: &str) -> Vec< Vec<i32> > {
    let mut result = vec![];
    for i in 0..128 {
        result.push(parse_hex(&day10::solve_part2(&format!("{}-{}", key, i), 256)));
    }
    result
}

fn count_used(grid: &Vec< Vec<i32> >) -> u32 {
    let mut count = 0;
    for row in grid {
        for item in row {
            count += item.count_ones();
        }
    }

    count
}

pub fn solve_part1(input: &str) -> u32 {
    let grid = generate_grid(input);
    count_used(&grid)
}

#[test]
fn test_examples_part1() {
    let input = "flqrgnkx";
    
    assert_eq!(solve_part1(input), 8108);
}

#[test]
fn test_given_input() {
    let input = "jxqlasbh";

    assert_eq!(solve_part1(input), 8140);
}
