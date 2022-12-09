use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_string(i: &str) -> (usize, usize, usize, usize) {
    let mut s = i.split(",");
    let elf1 = s.next().unwrap();
    let elf2 = s.next().unwrap();
    let mut s1 = elf1.split("-");
    let mut s2 = elf2.split("-");

    (
        s1.next().unwrap().parse::<usize>().unwrap(),
        s1.next().unwrap().parse::<usize>().unwrap(),
        s2.next().unwrap().parse::<usize>().unwrap(),
        s2.next().unwrap().parse::<usize>().unwrap()
    )
}

fn solution() -> (usize, usize) {
    let mut total = 0;
    let mut total2 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let (s1, s2, s3, s4) = parse_string(&l);

                // Solution 1
                if s1 >= s3 && s2 <= s4 || s1 <= s3 && s2 >= s4 {
                    total += 1;
                }

                // Solution 2
                if s1 >= s3 && s1 <= s4 ||
                    s2 >= s3 && s2 <= s4 ||
                    s3 >= s1 && s3 <= s2 ||
                    s4 >= s1 && s4 <= s2 {
                    total2 += 1;
                }
            }
        }
    }
    (total, total2)
}

fn main() {
    println!("Solution: {:?}", solution());
}
