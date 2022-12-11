use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

trait ThrowStrategy {
    fn calculate_new_woriness(&self, i: u64) -> u64;
    fn debug(&self);
}

struct MultiplayingStrategy {
    multiplier: u64,
}

impl ThrowStrategy for MultiplayingStrategy {
    fn calculate_new_woriness(&self, i: u64) -> u64 {
        self.multiplier * i
    }

    fn debug(&self) {
        println!("    Worry level is multiplied by {} to ?.", self.multiplier);
    }
}

struct AddingStrategy {
    addition: u64,
}

impl ThrowStrategy for AddingStrategy {
    fn calculate_new_woriness(&self, i: u64) -> u64 {
        self.addition + i
    }
    fn debug(&self) {
        println!("    Worry level is added by {} to ?.", self.addition);
    }
}

struct QuadraticStrategy {}

impl ThrowStrategy for QuadraticStrategy {
    fn calculate_new_woriness(&self, i: u64) -> u64 {
        i * i
    }
    fn debug(&self) {
        println!("    Worry level is squared to ?.");
    }
}

struct Monkey {
    pub items: Vec<u64>,
    operation: Box<dyn ThrowStrategy>,
    test: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: i32,
}

impl Monkey {
    fn parse_block(buf: &Vec<String>) -> Monkey {
        // Parse items
        //   Starting items: 60, 84, 84, 65
        let mut items = Vec::new();
        let mut s = buf[1].split(" ");
        (0..4).for_each(|_| { s.next(); });

        while let Some(i) = s.next() {
            let item = i.split(",").next().unwrap();
            items.push(item.parse::<u64>().expect(&format!("Not integer {} in {}", i, buf[1])));
        }

        //   Operation: new = old + 7
        s = buf[2].split(" ");
        (0..6).for_each(|_| { s.next(); });
        let operation: Box<dyn ThrowStrategy> = match (s.next().unwrap(), s.next().unwrap()) {
            ("+", v) => {
                Box::new(AddingStrategy { addition: v.parse::<u64>().expect(&format!("Not integer {}", v)) })
            }
            ("*", "old") => {
                Box::new(QuadraticStrategy {})
            }
            ("*", v) => {
                Box::new(MultiplayingStrategy { multiplier: v.parse::<u64>().unwrap() })
            }
            (o, v) => { panic!("Unknown operator {} {}", o, v); }
        };

        //   Test: divisible by 19
        s = buf[3].split(" ");
        (0..5).for_each(|_| { s.next(); });
        let test = s.next().unwrap().parse::<u64>().expect(&format!("Error on line {}", buf[3]));

        //     If true: throw to monkey 2
        s = buf[4].split(" ");
        (0..9).for_each(|_| { s.next(); });
        let true_monkey = s.next().unwrap().parse::<usize>().expect(&format!("Error on line {}", buf[4]));

        //     If false: throw to monkey 7
        s = buf[5].split(" ");
        (0..9).for_each(|_| { s.next(); });
        let false_monkey = s.next().unwrap().parse::<usize>().expect(&format!("Error on line {}", buf[5]));

        Monkey {
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
            inspection_count: 0,
        }
    }

    fn inspect_next_item(&mut self, w: u64, common_modulo: u64) -> Option<(usize, u64, u64)> {
        if let Some(i) = self.items.pop() {
            self.inspection_count += 1;
            let n = self.operation.calculate_new_woriness(i) / w % common_modulo;
            if n % self.test == 0 {
                return Some((self.true_monkey, i, n));
            } else {
                return Some((self.false_monkey, i, n));
            }
        }
        None
    }
}

fn parse_file() -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut buf = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(l) = line {
                buf.push(l);
                if i % 7 == 6 {
                    monkeys.push(Monkey::parse_block(&buf));
                    buf = Vec::new();
                }
            }
        }
    }
    monkeys
}

fn round(monkeys: &mut Vec<Monkey>, worriedness_factor: u64, common_modulo: u64, verbose: i32) {
    for i in 0..monkeys.len() {
        if verbose >= 3 { println!("Monkey {}", i); }
        while let Some((new_monkey, old_item, new_item)) = monkeys[i].inspect_next_item(worriedness_factor, common_modulo) {
            if verbose >= 3 { println!("  Monkey inspects an item with a worry level of {}", old_item); }
            if verbose >= 3 { println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", new_item); }
            if verbose >= 3 { println!("    Item with worry level {} is thrown to monkey {}.", new_item, new_monkey); }
            monkeys[new_monkey].items.push(new_item);
        }
    }
}

fn solution(iterations: i32, worriedness_factor: u64, verbose: i32) -> i64 {
    let mut monkeys = parse_file();
    let common_modulo = monkeys.iter().fold(1, |i, m| { i * m.test });
    if verbose >= 1 { println!("Common Modulo: {}", common_modulo); }
    for i in 0..iterations {
        if verbose >= 2 { println!("------ Round {} ------", i); }
        round(&mut monkeys, worriedness_factor, common_modulo, verbose);
        if verbose >= 2 {
            for m in monkeys.iter() {
                println!("Monkey x: {}", m.items.len());
            }
        }
    }
    let mut h = BinaryHeap::new();
    for (i, m) in monkeys.iter().enumerate() {
        if verbose >= 1 { println!("Monkey {} inspected items {} times.", i, m.inspection_count); }
        h.push(m.inspection_count as i64);
    }
    h.pop().unwrap() * h.pop().unwrap()
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
    if args.iter().find(|&arg| arg == "-vvv").is_some() {
        verbose = 3;
    }

    println!("Inspection count: {}", solution(20, 3, verbose));
    println!("Inspection count: {}", solution(10000, 1, verbose));
}

#[cfg(test)]
mod test {
    use crate::solution;

    #[test]
    fn test_solution() {
        let s = solution(20, 3, 0);
        assert_eq!(s, 55216);
    }

    #[test]
    fn test_solution2() {
        let s = solution(10000, 1, 0);
        assert_eq!(s < 14400239985, true);
        assert_eq!(s, 12848882750);
    }
}