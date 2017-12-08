use util;

#[derive(Debug)]
struct Program<'a> {
    name: String,
    weight: u32,
    children: Vec<Program<'a>>,
}

fn parse<'a>(input: &Vec< Vec<&str>>, index: usize) -> Program<'a> {
    let mut list = input[index].iter();
    let name = String::from(*list.next().unwrap());
    let weight = list.next().unwrap()
                    .trim_matches(|c| c == '(' || c == ')')
                    .parse::<u32>().unwrap();
    let mut children: Vec<&'a Program<'a>> = vec![];
    
    let children_idx: Vec<usize> = list.skip(1).map(|p| input.iter().position(|i| i[0] == p.trim_matches(|c| c == ',')).unwrap()).collect();
    
    children_idx.iter().for_each(|i| children.push(&parse(input, *i)));
    println!("after rt");
    

    Program{name, weight, children}
}

pub fn solve_part1<'a>(input: &str) -> String {
    let programs: Vec<Vec<&str>> = input.lines().map(|l| l.split_whitespace().collect()).collect();
    let mut index = 0;

    for i in 0..programs.len() {
        if programs[i].len() > 2 { 
            if programs.iter().find(|p| p.iter().skip(3).find(|n| *n.trim_matches(|c| c==',') == *programs[i][0]).is_some()).is_none() {
               index = i; 
            }
        }
    }

    parse(&programs, index).name
}

pub fn solve_part1_file(path: &str) -> String {
    solve_part1(&util::read_file(path).ok().unwrap())    
}

pub fn solve_part2(input: &str) -> u32 {
    32
}

#[test]
pub fn test_examples_part1() {
    let input =
"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    assert_eq!(solve_part1(&input), "tknk");
}

#[test]
pub fn test_examples_part2() {
    let input =
"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    assert_eq!(solve_part2(&input), 60);
}

#[test]
pub fn test_given_input() {
    let input = "inputs/day7.txt"; 

    assert_eq!(solve_part1_file(input), "vmpywg");
}
