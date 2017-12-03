fn find_distance(destination: i32) -> u32 {
    let mut ring = (destination as f64).sqrt().ceil() as i32;
    let ring_dist = (ring as f64 / 2.0).floor() as i32;
    if ring % 2 == 0 {
        ring += 1; 
    }

    let mut middle_dist = ring;
    for i in 0..4 {
        let middle: i32 = (ring*ring) - (ring_dist + ((ring - 1) * i));
        let dist = (destination as i64 - middle as i64).abs() as i32;
        middle_dist = middle_dist.min(dist);
    }

    (middle_dist + ring_dist) as u32
}

pub fn solve_part1(input: &str) -> u32 {
    find_distance(input.parse::<i32>().ok().unwrap())
}

#[test]
fn test_examples() {
    let destination = vec!["1", "12", "23", "1024"];
    let distance    = vec![0, 3,  2,  31];

    for (&dest, &dist) in destination.iter().zip(distance.iter()) {
        assert_eq!(solve_part1(dest), dist);
    }
}

#[test]
fn test_given_input() {
    let destination = "325489";

    assert_eq!(solve_part1(destination), 552);
}
