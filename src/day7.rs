use util;

#[derive(Debug)]
pub struct Program {
    name: String,
    weight: u32,
    children: Vec<Program>,
    children_weight: u32,
}

fn parse(input: &Vec< Vec<&str>>, index: usize) -> Program {
    let mut list = input[index].iter();
    let name = String::from(*list.next().unwrap());
    let weight = list.next().unwrap()
                    .trim_matches(|c| c == '(' || c == ')')
                    .parse::<u32>().unwrap();

    let children: Vec<Program> = list.skip(1).map(|p| parse(input, input.iter().position(|i| i[0] == p.trim_matches(|c| c == ',')).unwrap())).collect();
    
    let children_weight = children.iter().map(|c| c.weight + c.children_weight).fold(0, |acc, x| acc + x);

    Program{name, weight, children, children_weight}
}

fn find_unbalanced(part: &Program, expected: u32) -> (&Program, u32) {
    if part.children[0].weight + part.children[0].children_weight != part.children[1].weight + part.children[1].children_weight
        && part.children[0].weight + part.children[0].children_weight != part.children[2].weight + part.children[2].children_weight {
        return (&part.children[0], part.children[1].weight)
    } else if part.children[1].weight + part.children[1].children_weight != part.children[0].weight + part.children[0].children_weight
        && part.children[1].weight + part.children[0].children_weight != part.children[2].weight + part.children[2].children_weight {
        return (&part.children[1], part.children[1].weight)
    } else if part.children[2].weight + part.children[2].children_weight != part.children[0].weight + part.children[0].children_weight 
        && part.children[2].weight + part.children[2].children_weight != part.children[1].weight + part.children[1].children_weight {
        return (&part.children[2], part.children[0].weight)
    }

    println!("{:?} {:?}", part.name, expected);
    (part, expected)
}

fn check_balance(part: &Program, expected: u32) -> u32 {
    let (unbalanced, result) = find_unbalanced(part, expected);

    if unbalanced.name == part.name {
        return result;
    } else {
        return check_balance(unbalanced, result)
    }
}

pub fn solve_part1(input: &str) -> Program {
    let programs: Vec<Vec<&str>> = input.lines().map(|l| l.split_whitespace().collect()).collect();
    let mut index = 0;

    for i in 0..programs.len() {
        if programs[i].len() > 2 { 
            if programs.iter().find(|p| p.iter().skip(3).find(|n| *n.trim_matches(|c| c==',') == *programs[i][0]).is_some()).is_none() {
               index = i; 
            }
        }
    }

    parse(&programs, index)
}

pub fn solve_part1_file(path: &str) -> Program {
    solve_part1(&util::read_file(path).ok().unwrap())    
}

pub fn solve_part2(input: &str) -> u32 {
    let base = solve_part1(input);
    
    check_balance(&base, u32::max_value())
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

    assert_eq!(solve_part1(&input).name, "tknk");
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

    assert_eq!(solve_part1_file(input).name, "vmpywg");
}
