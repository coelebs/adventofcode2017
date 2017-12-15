use util;

static BLOCKSIZE: usize = 16;

fn parse(input: &str) -> Vec<u32> {
    input.split(',').map(|x| x.trim().parse::<u32>().unwrap()).collect()
}

fn parse_ascii(input: &str) -> Vec<u32> {
    input.chars().map(|c| c as u32).collect()
}

fn step(data: &mut Vec<usize>, index: &mut usize, rev_length: u32) {
    for i in 0..(rev_length as f32 / 2.0).floor() as usize {
        let len = data.len();
        let src = (*index + i) % len;
        let dest= (rev_length as usize - 1 + *index - i) % len;

        data.swap(src, dest);
    }
}

fn hash(input: &mut Vec<usize>, lengths: &Vec<u32>, index: &mut usize, skipsize: &mut usize) {
    for length in lengths {
        step(input, index, *length);
        *index = (*index + *length as usize + *skipsize) % input.len();
        *skipsize += 1;
    }

}

fn solve_part1(input: &str, data_length: usize) -> u32 {
    let input_lengths = parse(input);
    let mut data = vec![];
    let mut index = 0;
    let mut skipsize = 0;

    for i in 0..data_length {
        data.push(i);
    }

    hash(&mut data, &input_lengths, &mut index, &mut skipsize);

    (data[0] * data[1]) as u32
}

fn xor_blocks(input: &Vec<usize>) -> Vec<usize> {
    let mut dense_hash = vec![];
    for _i in 0..input.len()/BLOCKSIZE { dense_hash.push(0); }

    let mut i = 0;
    for val in dense_hash.iter_mut() {
        *val = input[i*BLOCKSIZE];
        for j in 1..BLOCKSIZE {
            let index = (i*BLOCKSIZE) + j;
            *val = *val ^ input[index];
        }
        i+= 1;
    }
    
    dense_hash
}

fn print_hex(input: &Vec<usize>) -> String {
    let mut result = String::new();
    for val in input.iter() {
        result.push_str(&format!("{:02x}", val));
    }
    result
}

fn solve_part2(input: &str, data_length: usize) -> String {
    let mut lengths = parse_ascii(input);
    lengths.extend([17, 31, 73, 47, 23].iter().cloned());
    let mut data = vec![];
    let mut index = 0;
    let mut skipsize = 0;

    for i in 0..data_length { data.push(i); }

    for _i in 0..64 {
        hash(&mut data, &lengths, &mut index, &mut skipsize);
    }

    print_hex(&xor_blocks(&data))
}

pub fn solve_part1_file(path: &str) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap(), 256)
}

pub fn solve_part2_file(path: &str) -> String {
    solve_part2(&util::read_file(path).ok().unwrap().trim(), 256)
}

#[test]
fn test_examples_part1() {
    let input_lengths = "3, 4, 1, 5";
    let data_length = 5;

    assert_eq!(solve_part1(input_lengths, data_length), 12);
}

#[test]
fn test_examples_part2() {
    let input = ["", "AoC 2017", "1,2,3", "1,2,4"];
    let result = [
"a2582a3a0e66e6e86e3812dcb672a272",
"33efeb34ea91902bb2f59c9920caa6cd",
"3efbe78a8d82f29979031a4aa0b16a9d",
"63960835bcdc130f0b66d7ff4f6a5a8e"];

    for (i, r) in input.iter().zip(result.iter()) {
        assert_eq!(solve_part2(i, 256), **r);
    }
}

#[test]
fn test_given_input() {
    let input = "inputs/day10.txt";
    
    assert_eq!(solve_part1_file(input), 212);
    assert_eq!(solve_part2_file(input), "96de9657665675b51cd03f0b3528ba26");
}
