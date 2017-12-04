fn find_ring(destination: u32) -> u32 {
    let mut ring = (destination as f64).sqrt().ceil() as u32;
    if ring % 2 == 0 {
        ring += 1; 
    }
    ring as u32
}

fn find_distance(destination: u32) -> u32 {
    let ring = find_ring(destination) as i32;
    let ring_dist = (ring as f64 / 2.0).floor() as i32;

    let mut middle_dist = ring;
    for i in 0..4 {
        let middle: i32 = (ring*ring) - (ring_dist + ((ring - 1) * i));
        let dist = (destination as i64 - middle as i64).abs() as i32;
        middle_dist = middle_dist.min(dist);
    }

    (middle_dist + ring_dist) as u32
}

fn find_neighbours(dest: u32) -> Vec<u32> {
    let mut neighbours = vec![dest - 1];
    let ring = find_ring(dest);
    let dist_on_ring = dest - (ring - 2).pow(2);

    let side = (dist_on_ring as f64 / (ring - 1) as f64).ceil() as u32; 
    let prev_corner = dest - ((ring - 1) * side) - ((ring - 3) * (4 - side)); //value diagonal back 

    if dist_on_ring == 1 {                      //first on new ring
        neighbours.push((ring - 4).pow(2) + 1); //neighbour is first on new ring previous cycle
    } else if dest == ring.pow(2) {
        neighbours.push((ring-2).pow(2));
        neighbours.push((ring-2).pow(2) + 1);
    } else if dist_on_ring % (ring - 1) == 0 {       //far corners
        neighbours.push(prev_corner);
    } else if dist_on_ring % (ring - 1) == ring - 2 && side != 4 {  //1 from far corner 
        neighbours.push(prev_corner);
        neighbours.push(prev_corner + 1);
    } else if dist_on_ring % (ring - 1) == 1 || dist_on_ring == 2 { //first on new block
        neighbours.push(prev_corner + 1);
        neighbours.push(prev_corner + 2);
        neighbours.push(dest - 2);
    } else {
        neighbours.push(prev_corner);
        neighbours.push(prev_corner + 1);
        neighbours.push(prev_corner + 2);
    }

    neighbours 
}

fn fill_spiral(limit: u32) -> Vec<u32> {
    let mut spiral = vec![1, 1, 2, 4, 5, 10];
    let mut i: u32 = spiral.len() as u32 + 1;

    while spiral.last() < Some(&limit) {
        let neighbours = find_neighbours(i); 
        let mut value = 0;

        for neighbour in neighbours {
            value += spiral[neighbour as usize - 1];
        }

        spiral.push(value);

        i += 1;
    }
    spiral
}

pub fn solve_part1(input: &str) -> u32 {
    find_distance(input.parse::<u32>().ok().unwrap())
}

pub fn solve_part2(input: &str) -> u32 {
    *fill_spiral(input.parse::<u32>().ok().unwrap()).last().unwrap()
}

#[test]
fn test_examples_part1() {
    let destination = vec!["1", "12", "23", "1024"];
    let distance    = vec![0, 3,  2,  31];

    for (&dest, &dist) in destination.iter().zip(distance.iter()) {
        assert_eq!(solve_part1(dest), dist);
    }
}

#[test]
fn test_examples_part2() {
    let spiral: Vec<u32> = 
        vec![1, 
             1, 2, 4, 5, 10, 11, 23, 25, 
             26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747, 806, 880, 931,
             957, 1968];

    assert_eq!(fill_spiral(*spiral.last().unwrap()), spiral);
}

#[test]
fn test_given_input() {
    let destination = "325489";

    assert_eq!(solve_part1(destination), 552);
    assert_eq!(solve_part2(destination), 330785);
}
