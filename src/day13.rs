use util;

fn parse(input: &str) -> Vec<usize> {
    let mut result = vec![];
    let mut count = 0;
    for line in input.lines() {
        let mut parts = line.split(':');
        let key = parts.next().unwrap().trim().parse::<usize>().unwrap();
        let value = parts.next().unwrap().trim().parse::<usize>().unwrap();
        while key > count {
            result.push(0);
            count += 1;
        }
        result.push(value);
    
        count += 1;
    }

    result
}

fn run_firewall(firewall: &Vec<usize>, delay: usize) -> (bool, usize) {
    let mut result = 0;
    let mut caught = false;
    for (layer, depth) in firewall.iter().enumerate() {  // we need the proper index, can't use filter
        if *depth > 0 {
            let len = if *depth > 2 {
                (*depth * 2) - 2
            } else {
                *depth
            };


            let mut pos = (delay + layer) % len; 

            if pos >= *depth {
                pos -= pos - (*depth - 2);
            }

            if pos == 0 {
                result += *depth * layer;  
                caught = true;
            }
        }
    }
    
    (caught, result)
}
fn solve_part1(input: &str) -> usize { let firewall = parse(input); 
    run_firewall(&firewall, 0).1
}

fn solve_part2(input: &str) -> usize {
    let firewall = parse(input);
    
    let mut caught = true;
    let mut delay = 0;

    while caught {
        delay += 1;

        caught = run_firewall(&firewall, delay).0;
    }

    delay 
}

pub fn solve_part1_file(path: &str) -> usize {
    solve_part1(&util::read_file(path).ok().unwrap())
}

pub fn solve_part2_file(path: &str) -> usize {
    solve_part2(&util::read_file(path).ok().unwrap())
}

#[test]
fn test_examples_part1() {
    let input = 
"0: 3
1: 2
4: 4
6: 4";

    assert_eq!(solve_part1(input), 24);
}

#[test]
fn test_examples_part2() {
    let input = 
"0: 3
1: 2
4: 4
6: 4";

    assert_eq!(solve_part2(input), 10);
}

#[test]
fn test_given_input() {
    let input = "inputs/day13.txt";

    assert_eq!(solve_part1_file(input), 2508);
    assert_eq!(solve_part2_file(input), 3913186);
}
