use util;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Duet {
    freq: i64,
    regs: HashMap<char, i64>,
    prog: Vec<String>,
    ip: i64,
    rx: Option<mpsc::Receiver<i64>>,
    tx: Option<mpsc::Sender<i64>>,
    send_count: i64,
}

impl Duet {
    fn snd(&mut self, reg: char) {
        let mut reg_s = String::new();
        reg_s.push(reg);
        let val = self.get_val(&reg_s);
        if self.tx.is_some() {
            self.tx.as_ref().unwrap().send(val).unwrap();
            self.send_count += 1;
        } else {
            self.freq = val;
        }
    }

    fn rcv(&mut self, reg: char) -> Option<i64> {
        if self.rx.is_some() {
            if let Ok(msg) = self.rx.as_ref().unwrap().
                                 recv_timeout(Duration::from_millis(800)) {
                self.regs.insert(reg, msg);
                None
            } else {
                Some(self.send_count)
            }
        } else if self.regs.get(&reg) != Some(&0) {
            self.regs.insert(reg, self.freq);
            Some(self.freq)
        } else {
            None
        }
    }

    fn set(&mut self, reg: char, arg: &str) {
        let val = self.get_val(arg);
        self.regs.insert(reg, val);
    }

    fn add(&mut self, reg: char, arg: &str) {
        let val = self.get_val(arg);
        let prev = self.regs.get_mut(&reg);
        if prev.is_some() {
            *prev.unwrap() += val;
        }
    }
    
    fn mul(&mut self, reg: char, arg: &str) {
        let val = self.get_val(arg);
        let prev = self.regs.get_mut(&reg);
        if prev.is_some() {
            *prev.unwrap() *= val;
        }
    }

    fn modd(&mut self, reg: char, arg: &str) {
        let val = self.get_val(arg);
        let prev = self.regs.get_mut(&reg);
        if prev.is_some() {
            *prev.unwrap() %= val;
        }
    }

    fn jgz(&mut self, reg: char, arg: &str) -> bool {
        let reg = self.get_val_reg(reg);
        let val = self.get_val(arg);
        if reg > 0 {
            self.ip += val;
            true
        } else {
            false 
        }
    }

    fn get_val_reg(&self, input: char) -> i64 {
        if let Some(result) = input.to_digit(10) {
            return result as i64;
        } else {
            return *self.regs.get(&input).unwrap_or(&0);
        }
    }

    fn get_val(&self, input: &str) -> i64 {
        if let Ok(result) = input.parse::<i64>() {
            return result;
        } else {
            return *self.regs.get(&input.chars().nth(0).unwrap()).unwrap_or(&0);
        }
    }

    fn run(&mut self) -> Result<i64, &'static str> {
        while self.ip < self.prog.len() as i64 && self.ip >= 0 {
            let tmp = self.prog[self.ip as usize].clone();
            let mut line = tmp.split_whitespace();
            let instruction = line.next();
            let reg = line.next().unwrap().chars().nth(0).unwrap();
            let arg = line.next();

            match instruction {
                Some("snd") =>    {self.snd(reg); self.ip += 1;} ,
                Some("set") =>    {self.set(reg, arg.unwrap());     self.ip += 1},
                Some("add") =>    {self.add(reg, arg.unwrap());     self.ip += 1},
                Some("mul") =>    {self.mul(reg, arg.unwrap());     self.ip += 1},
                Some("mod") =>    {self.modd(reg, arg.unwrap());    self.ip += 1},
                Some("rcv") =>    if let Some(result) = self.rcv(reg) {
                                      return Ok(result);
                                  } else {
                                      self.ip += 1;
                                  },
                Some("jgz") =>    if !self.jgz(reg, arg.unwrap()) {
                                      self.ip += 1;
                                  },
                _     =>    panic!("Unknown instruction"),
            }

        }

        Err("Reached end of program")
    }

    fn run_threaded(input: String, rx: mpsc::Receiver<i64>, tx: mpsc::Sender<i64>, p: i64) -> thread::JoinHandle<i64> {
        thread::spawn(move || {
            let mut duet = Duet::new(&input, Some(rx), Some(tx), Some(p));
            duet.run().ok().unwrap()
        })
    }


    fn new(input: &str, rx: Option<mpsc::Receiver<i64>>, tx: Option<mpsc::Sender<i64>>, p: Option<i64>) -> Duet {
        let mut result = Duet {
            freq: 0,
            regs: HashMap::new(),
            prog: input.lines().map(|s| String::from(s)).collect(),
            ip: 0,
            rx,
            tx,
            send_count: 0,
        };

        if p.is_some() {
            result.regs.insert('p', p.unwrap());
        }

        result
    }
}

fn solve_part1(input: &str) -> i64 {
    let mut duet = Duet::new(input, None, None, None);

    duet.run().ok().unwrap()
}

fn solve_part2(input: &str) -> i64 {
    let (tx0, rx0) = mpsc::channel::<i64>();
    let (tx1, rx1) = mpsc::channel::<i64>();

    Duet::run_threaded(String::from(input), rx0, tx1, 0); 
    Duet::run_threaded(String::from(input), rx1, tx0, 1).join().unwrap()
}

pub fn solve_part1_file(path: &str) -> i64 {
    solve_part1(&util::read_file(path).ok().unwrap().trim())
}

pub fn solve_part2_file(path: &str) -> i64 {
    solve_part2(&util::read_file(path).ok().unwrap().trim())
}

#[test]
fn test_examples_part1() {
    let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    assert_eq!(solve_part1(input), 4);
}

#[test]
fn test_examples_part2() {
    let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    assert_eq!(solve_part2(input), 3);
}

#[test]
fn test_given_input() {
    let input = "inputs/day18.txt";

    assert_eq!(solve_part1_file(input), 9423);
    assert_eq!(solve_part2_file(input), 7620);
}
