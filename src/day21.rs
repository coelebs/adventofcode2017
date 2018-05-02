use util;

struct Grid {
    image: String,
    sections: Vec<Vec<String>>,
    size: usize,
}

struct Rule {
    input: Vec<String>,
    output: Vec<String>,
}

impl Grid {
    fn split(&mut self) {
        self.sections.clear();

        let image_grid: Vec<_> = self.image.lines().collect();
        
        self.size = Grid::size(image_grid.len()).unwrap();

        println!("{:?} {:?}", image_grid.len(), self.size);

         for i in 0..(image_grid.len() / self.size) {

            for j in 0..(image_grid[i].len() / self.size) {
                let mut section: Vec<String> = vec![];

                for k in 0..self.size {
                    let row = (i * self.size) + k;
                    let col = j * self.size;

                    section.push(String::from(&image_grid[row][col..col+self.size]));
                }
                
                self.sections.push(section);
            }
        }
    }

    fn size(len: usize) -> Option<usize> {
        println!("sze {:?}", len);
        if len == 1 {
            Some(1)
        } else if len % 2 == 0 {
            Some(2)
        } else if len % 3 == 0 {
            Some(3)
        } else {
            None
        }
    }

    fn merge(&mut self) {
        let mut lines = vec![];
        let section_size = self.sections[0].len();

        //self.size = Grid::size(self.sections.len()).unwrap();

        for i in 0..self.sections.len() {
            let row = (i / self.size) * self.size;
            println!("{:?} {:?} {:?} {:?}", row, i, self.size, section_size);

            for j in 0..self.sections[i].len() {
                if (row + j) >= lines.len() {
                    lines.push(String::new());
                }

                lines[row+j].push_str(&self.sections[i][j]);
            }
        }

        self.image = lines.join("\n");
    }

    fn step(&mut self, rules: &Vec<Rule>) {
        let mut sections = vec![];
        let mut prev = 0;
        for section in self.sections.iter() {
            for rule in rules {
                if let Some(output) = rule.apply(&section) {
                    sections.push(output);
                    break;
                }
            }

            if prev == sections.len() {
                panic!("No recipe found!");
            }

            prev = sections.len();
        } 

        self.sections = sections;
    }
}

impl Rule {
    fn parse(input: &str) -> Rule {
        let mut iter = input.split("=>");
        let input = iter.next().unwrap().trim().split("/").map(|s| String::from(s)).collect();
        let output = iter.next().unwrap().trim().split("/").map(|s| String::from(s)).collect();

        Rule { input, output }
    }

    fn apply(&self, input: &Vec<String>) -> Option<Vec<String>> {
        let mut orig = input.to_vec();
        for _i in 0..4 {
            let mut tmp = orig.to_vec();
            if tmp == self.input {
                return Some(self.output.to_vec());
            }

            tmp.reverse();
            if tmp == self.input {
                return Some(self.output.to_vec());
            }

            tmp = flip_horizontally(&orig);
            if tmp == self.input {
                return Some(self.output.to_vec());
            }

            orig = rotate(orig);
        }

        None
    }
}

fn flip_horizontally(section: &Vec<String>) -> Vec<String> {
    let mut result = vec![];
    for row in section.iter() {
        result.push(row.chars().rev().collect());
    }
    result
}

fn rotate(section: Vec<String>) -> Vec<String> {
    let mut result = vec![]; 
    let size = section.len() - 1;

    for i in 0..section.len() {
        result.push(String::new());
        for j in 0..section[i].len() {
            result[i].push(section[size - j].chars().nth(i).unwrap()); 
        }
    }

    result
}

fn parse_rules(input: &str) -> Vec<Rule> {
    let mut result = vec![];

    for line in input.lines() {
        result.push(Rule::parse(line));
    }

    result
}

fn solve_part1(input: &str, rounds: usize) -> usize {
    let rules = parse_rules(input);
    let mut grid = Grid{image: String::from(".#.\n..#\n###"), sections: vec![], size: 0};

    for _i in 0..rounds {
        grid.split();
        grid.step(&rules);
        grid.merge();
        println!("{:?}\n{}", grid.sections, grid.image);
    }


    grid.image.chars().fold(0, |acc, c| if c == '#' { acc + 1 } else { acc })
}

pub fn solve_part1_file(path: &str) -> usize {
    solve_part1(&util::read_file(path).ok().unwrap(), 5)
}

#[test]
fn test_examples_part1() {
    let input = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";

    assert_eq!(solve_part1(input, 2), 12);
}
