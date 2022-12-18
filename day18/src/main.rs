use std::collections::{BTreeMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_string(input: &str) -> (i32, i32, i32) {
    let mut s = input.split(",");
    (
        s.next().unwrap().parse::<i32>().unwrap(),
        s.next().unwrap().parse::<i32>().unwrap(),
        s.next().unwrap().parse::<i32>().unwrap()
    )
}

fn parse_file(file: &str) -> BTreeMap<(i32, i32, i32), i32> {
    let mut lava = BTreeMap::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            lava.insert(parse_string(&line), 1);
        }
    }
    lava
}

fn solution(file: &str) -> i32 {
    let mut surfaces = 0;
    let lava = parse_file(file);

    for ((x, y, z), _) in lava.iter() {
        if lava.get(&(*x + 1, *y, *z)).is_none() {
            surfaces += 1;
        }
        if lava.get(&(*x - 1, *y, *z)).is_none() {
            surfaces += 1;
        }
        if lava.get(&(*x, *y + 1, *z)).is_none() {
            surfaces += 1;
        }
        if lava.get(&(*x, *y - 1, *z)).is_none() {
            surfaces += 1;
        }
        if lava.get(&(*x, *y, *z + 1)).is_none() {
            surfaces += 1;
        }
        if lava.get(&(*x, *y, *z - 1)).is_none() {
            surfaces += 1;
        }
    }
    surfaces
}

fn fill(mut lava: BTreeMap<(i32, i32, i32), i32>) -> BTreeMap<(i32, i32, i32), i32> {
    let mut steam_expansion = Vec::new();
    steam_expansion.push((0, 0, 0));

    while let Some((x, y, z)) = steam_expansion.pop() {
        if x >= -5 && x <= 25 &&
            y >= -5 && y <= 25 &&
            z >= -5 && z <= 25 {
            if let None = lava.get(&(x, y, z)) {
                lava.insert((x, y, z), 2);
                steam_expansion.push((x + 1, y, z));
                steam_expansion.push((x - 1, y, z));
                steam_expansion.push((x, y + 1, z));
                steam_expansion.push((x, y - 1, z));
                steam_expansion.push((x, y, z + 1));
                steam_expansion.push((x, y, z - 1));
            }
        }
    }
    lava
}

fn solution2(file: &str) -> i32 {
    let mut surfaces = 0;
    let lava = parse_file(file);
    let map = fill(lava.clone());

    for ((x, y, z), _) in lava.iter() {
        if let Some(x) = map.get(&(*x + 1, *y, *z)) {
            if *x == 2 {
                surfaces += 1;
            }
        }
        if let Some(x) = map.get(&(*x - 1, *y, *z)) {
            if *x == 2 {
                surfaces += 1;
            }
        }
        if let Some(x) = map.get(&(*x, *y + 1, *z)) {
            if *x == 2 {
                surfaces += 1;
            }
        }
        if let Some(x) = map.get(&(*x, *y - 1, *z)) {
            if *x == 2 {
                surfaces += 1;
            }
        }
        if let Some(x) = map.get(&(*x, *y, *z + 1)) {
            if *x == 2 {
                surfaces += 1;
            }
        }
        if let Some(x) = map.get(&(*x, *y, *z - 1)) {
            if *x == 2 {
                surfaces += 1;
            }
        }
    }
    surfaces
}

fn main() {
    println!("Solution: {}", solution("test.txt"));
    println!("Solution: {}", solution2("test.txt"));

    println!("Solution: {}", solution("input.txt"));
    // 2057 wrong
    println!("Solution: {}", solution2("input.txt"));
}
