use util;

struct Coordanites {
    row: i32,
    col: i32,
}

enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl Coordanites {
    fn new() -> Coordanites  {
        Coordanites{ row: 0, col: 0}
    }

    fn step(&mut self, dir: &Direction) {
        match *dir {
            Direction::North        => {
                self.row += -1;
            }
            Direction::South        => {
                self.row += 1;
            }

            Direction::SouthEast    => {
                self.col += 1;
            }
            Direction::NorthWest    => {
                self.col += -1;
            }


            Direction::NorthEast    => {
                self.row += -1;
                self.col += 1;
            }
            Direction::SouthWest    => {
                self.row += 1;
                self.col += -1;
            }
        }
    }
}

fn parse(input: &str) -> Vec<Direction> {
    let mut directions = vec![];
    for step in input.trim().split(',') {
        directions.push(match step {
            "n"  => Direction::North,
            "ne" => Direction::NorthEast,
            "se" => Direction::SouthEast,
            "s"  => Direction::South,
            "sw" => Direction::SouthWest,
            "nw" => Direction::NorthWest,
            _    => panic!(),
        });
    }
    directions
}

fn calculate_distance(a: &Coordanites, b: &Coordanites) -> u32 {
    ((a.col - b.col).abs() 
     + (a.col + a.row - b.col - b.row).abs()
     + (a.row - b.row).abs()) as u32 / 2
}

fn exec_steps(directions: &Vec<Direction>, max_dist: &mut u32) -> Coordanites {
    let mut cords = Coordanites::new();
    directions.iter().for_each(|dir| {
        cords.step(dir);
        let dist = calculate_distance(&Coordanites::new(), &cords);
        if dist > *max_dist {
            *max_dist = dist;
        }
    });
    cords
}

fn solve_part1(input: &str) -> u32 {
    let mut max_dist = 0;
    let cords = exec_steps(&parse(input), &mut max_dist);

    calculate_distance(&Coordanites::new(), &cords)
}

pub fn solve_part1_file(path: &str) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap())
}

fn solve_part2(input: &str) -> u32 {
    let mut max_dist = 0;
    exec_steps(&parse(input), &mut max_dist);

    max_dist
}

pub fn solve_part2_file(path: &str) -> u32 {
    solve_part2(&util::read_file(path).ok().unwrap())
}

#[test]
fn test_examples_part1() {
    let input = ["ne,ne,ne", "ne,ne,sw,sw", "ne,ne,s,s", "se,sw,se,sw,sw,sw,nw"];
    let result = [3, 0, 2, 4];

    for (i, r) in input.iter().zip(result.iter()) {
        assert_eq!(solve_part1(i), *r);
    }
}

#[test] 
fn test_given_input() {
    let input = "inputs/day11.txt";

    assert_eq!(solve_part1_file(input), 682);
    assert_eq!(solve_part2_file(input), 1406);
}
