use util;

#[derive(Debug)]
struct Particle {
    pos: Coordinate,
    acc: Coordinate,
    vel: Coordinate,
}

#[derive(Debug,PartialEq)]
struct Coordinate {
    x: i32,
    y: i32, 
    z: i32,
}

impl Particle {
    fn weight(&self) -> u32 {
        (self.acc.x.abs() as u32 
         + self.acc.y.abs() as u32 
         + self.acc.z.abs() as u32) 
    }

    fn step(&mut self) {
        self.vel.add(&self.acc);
        self.pos.add(&self.vel);
    }

    fn collides(&self, b: &Particle) -> bool {
        self.pos == b.pos
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

    fn add(&mut self, b: &Coordinate) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }
}

fn parse(input: &str) -> Vec<Particle> {
    let mut result = vec![];
    for line in input.lines() {
        let line = line.trim();
        let mut iter = line.split(", ");

        let pos = Coordinate::parse(&iter.next().unwrap()[2..]);
        let vel = Coordinate::parse(&iter.next().unwrap()[2..]);
        let acc = Coordinate::parse(&iter.next().unwrap()[2..]);
        
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

fn step(particles: &mut Vec<Particle>) {
    particles.iter_mut().for_each(|p| p.step());

    let mut dup = vec![];

    for i in 0..particles.len() {
        for j in i+1..particles.len() {
            if particles[i].collides(&particles[j]) {
                dup.push(i);
                dup.push(j);
            }
        }
    }

    dup.sort();
    dup.dedup();

    let mut removed = 0;
    for i in dup.iter() {
        particles.remove(*i - removed);
        removed += 1;
    }
}

fn solve_part1(input: &str) -> usize {
    let particles = parse(input);
    find_slowest(&particles)
}

pub fn solve_part1_file(path: &str) -> usize {
    solve_part1(&util::read_file(path).ok().unwrap())
}

fn solve_part2(input: &str) -> usize {
    let mut particles = parse(input);
    let mut no_change = 0;
    let mut prev = 0;

    while no_change < 50 {
        step(&mut particles);

        if prev == particles.len() {
            no_change += 1;
        } else {
            prev = particles.len();
            no_change = 0;
        }
    }

    particles.len()
}

pub fn solve_part2_file(path: &str) -> usize {
    solve_part2(&util::read_file(path).ok().unwrap())
}
 
#[test]
fn test_examples_part1() {
    let input = "p=<3,0,0>, v=<-2,0,0>, a=<-1,0,0> 
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    assert_eq!(solve_part1(input), 0);
}

#[test]
fn test_examples_part2() {
    let input = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>    
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

    assert_eq!(solve_part2(input), 1);
}

#[test]
fn test_given_input() {
    let input = "inputs/day20.txt";

    assert_eq!(solve_part1_file(input), 144);
    assert_eq!(solve_part2_file(input), 477);
}
