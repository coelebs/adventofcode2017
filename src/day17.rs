fn step(data: &mut Vec<u32>, index: &mut usize, round: u32, key: u32) {
    *index = (*index + key as usize) % data.len();
    *index += 1;
    
    data.insert(*index, round);
}

pub fn solve_part1(key: u32) -> u32 {
    let mut index = 0;
    let mut data = vec![0];

    for round in 1..2018 {
        step(&mut data, &mut index, round, key);
    }

    *data.iter().nth((index + 1) % data.len()).unwrap()
}

pub fn solve_part2(key: u32) -> u32 {
    let mut index = 0;
    let mut result = 0;

    for round in 1..50000001 {
        index = ((index + key as usize) % round) + 1;
        
        if index == 1 {
            result = round;
        }
    }

    result as u32
}

#[test]
fn test_examples_part1() {
    let input = 3;

    assert_eq!(solve_part1(input), 638);
}

#[test]
fn test_given_input() {
    let input = 380;

    assert_eq!(solve_part1(input), 204);
    assert_eq!(solve_part2(input), 28954211);
}
