use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
 *     [H]                 [Z]         [J]
 *     [L]     [W] [B]     [G]         [R]
 *     [R]     [G] [S]     [J] [H]     [Q]
 *     [F]     [N] [T] [J] [P] [R]     [F]
 *     [B]     [C] [M] [R] [Q] [F] [G] [P]
 *     [C] [D] [F] [D] [D] [D] [T] [M] [G]
 *     [J] [C] [J] [J] [C] [L] [Z] [V] [B]
 *     [M] [Z] [H] [P] [N] [W] [P] [L] [C]
 *      1   2   3   4   5   6   7   8   9
*/
fn setup() -> [Vec<char>; 9] {
    let mut i: [Vec<char>; 9] = Default::default();

    i[8].push('C');    i[8].push('B');    i[8].push('G');    i[8].push('P');
    i[8].push('F');    i[8].push('Q');    i[8].push('R');    i[8].push('J');

    i[7].push('L');    i[7].push('V');    i[7].push('M');    i[7].push('G');

    i[6].push('P');    i[6].push('Z');    i[6].push('T');    i[6].push('F');
    i[6].push('R');    i[6].push('H');

    i[5].push('W');    i[5].push('L');    i[5].push('D');    i[5].push('Q');
    i[5].push('P');    i[5].push('J');    i[5].push('G');    i[5].push('Z');

    i[4].push('N');    i[4].push('C');    i[4].push('D');    i[4].push('R');
    i[4].push('J');

    i[3].push('P');    i[3].push('J');    i[3].push('D');    i[3].push('M');
    i[3].push('T');    i[3].push('S');    i[3].push('B');

    i[2].push('H');    i[2].push('J');    i[2].push('F');    i[2].push('C');
    i[2].push('N');    i[2].push('G');    i[2].push('W');

    i[1].push('Z');    i[1].push('C');    i[1].push('D');

    i[0].push('M');    i[0].push('J');    i[0].push('C');    i[0].push('B');
    i[0].push('F');    i[0].push('R');    i[0].push('L');    i[0].push('H');

    i
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_string(i: &str) -> (usize, usize, usize) {
    let mut s = i.split(" ");
    s.next();
    let count = s.next().unwrap().parse::<usize>().unwrap();
    s.next();
    let from = s.next().unwrap().parse::<usize>().unwrap();
    s.next();
    let to = s.next().unwrap().parse::<usize>().unwrap();

    (
        count,
        from-1,
        to-1,
    )
}

fn solution() -> [Vec<char>; 9] {
    let mut s = setup();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let (count, from, to) = parse_string(&l);
                // println!("Moving {} from {} to {}", count, from + 1, to + 1);
                for _i in 0..count {
                    // println!("Moving");
                    let c = s[from].pop().unwrap();
                    s[to].push(c);
                }
            }
        }
    }
    s
}

fn solution2() -> [Vec<char>; 9] {
    let mut s = setup();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let (count, from, to) = parse_string(&l);
                // println!("Moving {} from {} to {}", count, from + 1, to + 1);
                let mut stack = Vec::new();
                for _i in 0..count {
                    // println!("Moving");
                    let c = s[from].pop().unwrap();
                    stack.push(c);
                }
                stack.reverse();
                for c in stack {
                    s[to].push(c);
                }
            }
        }
    }
    s
}

fn main() {
    let mut s = solution();
    println!("Solution: ");
    for i in s.iter_mut() {
        print!("{}", i.pop().unwrap());
    }
    println!();

    let mut s = solution2();
    println!("Solution: ");
    for i in s.iter_mut() {
        print!("{}", i.pop().unwrap());
    }
    println!();
}
