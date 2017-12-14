use util;

fn filter_escape(input: &mut String) {
    let mut escapes: Vec<usize> = input.chars().enumerate().map(|c| if c.1 == '!' { c.0 } else { usize::max_value() }).collect();

    escapes = escapes.iter().filter(|x| **x < usize::max_value()).map(|x| *x).collect();
    let mut removed = 0;
    
    for i in 1..escapes.len() {
        if escapes[i - removed] - escapes[i - 1 - removed] == 1 {
            escapes.remove(i - removed); 
            removed += 1;
        }
    }

    removed = 0;
    for i in 0..escapes.len() {
        input.remove(escapes[i] - removed);
        removed += 1;
        input.remove(escapes[i] + 1 - removed);
        removed += 1;
    }
}

fn filter_garbage(input: &mut String) -> u32 {
    let mut start_garbage: Vec<usize> = input.chars().enumerate().map(|c| if c.1 == '<' { c.0 } else { usize::max_value() }).collect();

    let mut end_garbage: Vec<usize> = input.chars().enumerate().map(|c| if c.1 == '>' { c.0 } else { usize::max_value() }).collect();


    start_garbage = start_garbage.iter().filter(|x| **x < usize::max_value()).map(|x| *x).collect();
    end_garbage = end_garbage.iter().filter(|x| **x < usize::max_value()).map(|x| *x).collect();

    let mut removed = 0; 
    let mut removed_groups = 0;
    let mut prev_end:usize = 0;
    for end in end_garbage.iter() {
        let start = start_garbage.iter().find(|&x| x < end && *x >= prev_end).unwrap();

        for n in *start..*end + 1 {
            input.remove(n - removed);

            removed += 1;
        }

        prev_end = *end;

        removed_groups += 2;
    }

    (removed - removed_groups) as u32
}

fn score_groups(input: &String, level: u32) -> u32 {
    let mut groups = 0;
    let mut depth = 0;
    let mut open_for_group = true;

    let mut subgroup = String::new();

    for c in input.chars() {
        if c == '{' {
            depth += 1;
            if depth > 1 {
                subgroup.push(c);
            }
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                groups += 1;
                open_for_group = false;
                subgroup.push(',');
            } else {
                subgroup.push(c);
            }
        } else if c == ',' && !open_for_group {
            open_for_group = true;
        }
    }

    let subgroup_open = subgroup.chars().find(|&c| c == '{');
    let subgroup_close = subgroup.chars().find(|&c| c == '}');

    if subgroup.len() > 0 && subgroup_open.is_some() && subgroup_close.is_some() {
        (groups * level) + score_groups(&subgroup, level + 1)
    } else {
        (groups * level)
    }
}

pub fn solve_part1(input: &str) -> u32 {
    let mut filtered = String::from(input);
    filter_escape(&mut filtered);
    filter_garbage(&mut filtered);
    score_groups(&filtered, 1)
}

pub fn solve_part2(input: &str) -> u32 {
    let mut filtered = String::from(input);
    filter_escape(&mut filtered);
    filter_garbage(&mut filtered)
}

pub fn solve_part1_file(path: &str) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap())
}

pub fn solve_part2_file(path: &str) -> u32 {
    solve_part2(&util::read_file(path).ok().unwrap())
}

#[test]
fn test_examples_part1() {
    let input = vec![
"{}",
"{{{}}}",
"{{},{}}",
"{{{},{},{{}}}}",
"{<{},{},{{}}>}",
"{{<ab>},{<ab>},{<ab>},{<ab>}}",
"{{<!!>},{<!!>},{<!!>},{<!!>}}",
"{{<a!>},{<a!>},{<a!>},{<ab>}}"];

    let result: Vec<u32> = vec![1, 6, 5, 16, 1, 9, 9, 3];

    for (i, r) in input.iter().zip(result.iter()) {
        assert_eq!(solve_part1(i), *r);
    }
}

#[test]
fn test_examples_part2() {
    let input = vec![
"<>",
"<random characters>",
"<<<<>",
"<{!>}>",
"<!!>",
"<!!!>>",
"<{o\"i!a,<{i<a>"];

    let result = vec![0, 17, 3, 2, 0, 0, 10];

    for (i, r) in input.iter().zip(result.iter()) {
        assert_eq!(solve_part2(i), *r);
    }
}

#[test]
fn test_given_input() {
    let input = "inputs/day9.txt";

    assert_eq!(solve_part1_file(input), 17537);
    assert_eq!(solve_part2_file(input), 7539);
}
