use util;

#[derive(Debug)]
struct Particle {
    pos: Coordinate,
    acc: Coordinate,
    vel: Coordinate,
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32, 
    z: i32,
}

impl Particle {
    fn weight(&self) -> u32 {
        (self.vel.x.abs() as u32 
         + self.vel.y.abs() as u32 
         + self.vel.z.abs() as u32) 
    }
}

impl Coordinate {
    fn parse(input: &str) -> Coordinate {
        let mut iter = input.trim_matches(|c| c == '<' || c == '>').split(',');
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        let z = iter.next().unwrap().parse::<i32>().unwrap();

        Coordinate{x,y,z}
    }
}

fn parse(input: &str) -> Vec<Particle> {
    let mut result = vec![];
    for line in input.lines() {
        let line = line.trim();
        let mut iter = line.split(", ");

        let pos = Coordinate::parse(&iter.next().unwrap()[2..]);
        let acc = Coordinate::parse(&iter.next().unwrap()[2..]);
        let vel = Coordinate::parse(&iter.next().unwrap()[2..]);
        
        result.push(Particle{pos,acc,vel});
    }

    result
}

fn find_slowest(particles: &Vec<Particle>) -> usize {
    let mut slowest = 0;

    for (i, particle) in particles.iter().enumerate() {
        if particle.weight() < particles[slowest].weight() {
            slowest = i;
        }
    }

    slowest
}

fn solve_part1(input: &str) -> usize {
    let particles = parse(input);
    find_slowest(&particles)
}

pub fn solve_part1_file(path: &str) -> usize {
    solve_part1(&util::read_file(path).ok().unwrap())
}

#[test]
fn test_examples_part1() {
    let input = "p=<3,0,0>, v=<-2,0,0>, a=<-1,0,0> 
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    assert_eq!(solve_part1(input), 0);
}

#[test]
fn test_given_input() {
    let input = "inputs/day20.txt";

    assert_eq!(solve_part1_file(input), 144);
}
