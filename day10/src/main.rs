use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::Instruction::{AddX, Noop};

enum Instruction {
    AddX(i32),
    Noop,
}

struct Crt {
    buf: [char; 240],
}

impl Crt {
    pub fn new() -> Crt {
        Crt { buf: [' '; 240] }
    }

    pub fn display(&self) {
        for i in 0..240 {
            if i % 40 == 0 {
                println!();
            }
            print!("{}", self.buf[i]);
        }
        println!();
    }

    pub fn set_pixel(&mut self, cycle: i32, sprite: i32) {
        if ((cycle) % 40).abs_diff(sprite + 1) <= 1 {
            self.buf[(cycle - 1) as usize] = '#';
        } else {
            self.buf[(cycle - 1) as usize] = '.';
        }
    }
}

struct Cpu {
    x: i32,
    cycle: i32,
    pub crt: Crt,
    pub total_strength: i32,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { crt: Crt::new(), x: 1, cycle: 1, total_strength: 0 }
    }

    fn execute(&mut self, i: Instruction) {
        self.next_cycle();
        match i {
            AddX(v) => {
                self.next_cycle();
                self.x += v;
            }
            Noop => {}
        }
    }

    fn next_cycle(&mut self) {
        self.crt.set_pixel(self.cycle, self.x);
        if self.cycle == 20 || (self.cycle - 20) % 40 == 0 {
            self.emit_signal_strength();
        }
        self.cycle += 1;
    }

    fn emit_signal_strength(&mut self) {
        self.total_strength += self.cycle * self.x;
        println!("Strength @ {} = {}: {}", self.cycle, self.x, self.cycle * self.x);
    }
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_string(i: &str) -> Instruction {
    let mut s = i.split(" ");
    let instruction = s.next().unwrap();
    if instruction == "addx" {
        return AddX(s.next().unwrap().parse::<i32>().unwrap());
    }
    Noop
}

fn solution(verbose: usize) -> i32 {
    let mut cpu = Cpu::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if verbose >= 2 { println!("{}", l); }
                let i = parse_string(&l);
                cpu.execute(i);
                if verbose >= 1 { cpu.emit_signal_strength() }
            }
        }
    }
    cpu.crt.display();
    cpu.total_strength
}

fn main() {
    let mut verbose = 0;
    let args: Vec<String> = env::args().collect();
    if args.iter().find(|&arg| arg == "-v").is_some() {
        verbose = 1;
    }
    if args.iter().find(|&arg| arg == "-vv").is_some() {
        verbose = 2;
    }
    println!("Solution {}", solution(verbose));
}
