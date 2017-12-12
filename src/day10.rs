use util;

fn parse(input: &str) -> Vec<u32> {
    input.split(',').map(|x| x.trim().parse::<u32>().unwrap()).collect()
}

fn step(data: &mut Vec<usize>, index: usize, rev_length: u32) {
    for i in 0..(rev_length as f32 / 2.0).floor() as usize {
        let len = data.len();
        data.swap((index + i) % len, (rev_length as usize  + index - i) % len);
    }
}

fn solve_part1(input: &str, data_length: usize) -> u32 {
    let input_lengths = parse(input);
    let mut data = vec![];
    let mut index = 0;
    let mut skip_size = 0;

    for i in 0..data_length {
        data.push(i);
    }

    for length in input_lengths {
        step(&mut data, index, length);
        index = length as usize + skip_size;
        skip_size += 1;
        println!("{:?}", data);
    }

    (data[0] * data[1]) as u32
}

pub fn solve_part1_file(path: &str) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap(), 255)
}

#[test]
fn test_examples_part1() {
    let input_lengths = "3, 4, 1, 5";
    let data_length = 5;

    assert_eq!(solve_part1(input_lengths, data_length), 12);
}
