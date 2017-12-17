use util;

struct Distance {
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

impl Distance {
    fn new() -> Distance  {
        Distance{ row: 0, col: 0}
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

fn calculate_distance(a: &Distance, b: &Distance) -> u32 {
    ((a.col - b.col).abs() 
     + (a.col + a.row - b.col - b.row).abs()
     + (a.row - b.row).abs()) as u32 / 2
}

fn exec_steps(directions: &Vec<Direction>) -> Distance {
    let mut dist = Distance::new();
    directions.iter().for_each(|dir| dist.step(dir));
    dist
}

fn solve_part1(input: &str) -> u32 {
    let directions = parse(input); 
    let distance = exec_steps(&directions);

    calculate_distance(&Distance::new(), &distance)
}

pub fn solve_part1_file(path: &str) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap())
}

#[test]
fn test_examples_part1() {
    let input = ["ne,ne,ne", "ne,ne,sw,sw", "ne,ne,s,s", "se,sw,se,sw,sw,sw,nw"];
    let result = [3, 0, 2, 4];

    for (i, r) in input.iter().zip(result.iter()) {
        assert_eq!(solve_part1(i), *r);
    }
}
