use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static VERBOSE: u32 = 0;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_file(file: &str, start: char, direction: i32) -> (Vec<Vec<u8>>, Option<(usize, usize)>) {
    let mut map = Vec::new();
    let mut s = None;
    if let Ok(lines) = read_lines(file) {
        for (y, line) in lines.enumerate() {
            if let Ok(l) = line {
                let mut row = Vec::new();
                for (x, c) in l.bytes().enumerate() {
                    if c == start as u8 {
                        s = Some((x, y));
                        if direction > 0 {
                            row.push('a' as u8);
                        } else {
                            row.push('z' as u8);
                        }
                    } else {
                        row.push(c);
                    }
                }
                map.push(row);
            }
        }
    }
    (map, s)
}

fn can_travel(a: u8, mut b: u8, direction: i32) -> bool {
    if b == 'E' as u8 {
        b = 'z' as u8;
    }
    if direction > 0 {
        a + 1 >= b
    } else {
        a - 1 <= b
    }
}

fn get_neighbours(map: &Vec<Vec<u8>>, x: usize, y: usize, direction: i32) -> Vec<(usize, usize)> {
    let mut n = Vec::new();

    if VERBOSE >= 2 {
        if x < map[0].len()-1 {
            println!("Scanning x+1: {} >= {}", map[y][x] as char, map[y][x + 1] as char);
        }
        if x > 0 {
            println!("Scanning x-1: {} >= {}", map[y][x] as char, map[y][x - 1] as char);
        }
        if y < map.len() -1 {
            println!("Scanning y+1: {} >= {}", map[y][x] as char, map[y + 1][x] as char);
        }
        if y > 0 {
            println!("Scanning y-1: {} >= {}", map[y][x] as char, map[y - 1][x] as char);
        }
    }
    if x < map[0].len()-1 && can_travel(map[y][x], map[y][x + 1], direction) {
        n.push((x + 1, y));
    }
    if x > 0 && can_travel(map[y][x], map[y][x - 1], direction) {
        n.push((x - 1, y));
    }
    if y < map.len() -1 && can_travel(map[y][x], map[y + 1][x], direction) {
        n.push((x, y + 1));
    }
    if y > 0 && can_travel(map[y][x], map[y - 1][x], direction) {
        n.push((x, y - 1));
    }
    n
}

fn search(map: &mut Vec<Vec<u8>>, x: usize, y: usize, end: char, direction: i32) -> Option<Vec<(usize, usize)>> {
    let mut q = VecDeque::new();

    let mut path = Vec::new();
    path.push((x, y));
    q.push_back(path);

    while let Some(path) = q.pop_front() {
        let (x, y) = path.get(path.len() - 1).unwrap();
        if VERBOSE >= 2 { println!("standing at ({}, {})", x, y); }
        if map[*y][*x] == end as u8 {
            if VERBOSE >= 2 { println!("Solution found {}, {}", map[*y][*x], end as u8); }
            return Some(path);
        }
        if map[*y][*x] != 255 && map[*y][*x] != 0 {
            for (nx, ny) in get_neighbours(map, *x, *y, direction) {
                if VERBOSE >= 2 { println!("N: {} {} ", nx, ny); }
                let mut p = path.clone();
                p.push((nx, ny));
                q.push_back(p);
            }
            if direction > 0 {
                map[*y][*x] = 255;
            } else {
                map[*y][*x] = 0;
            }
        }
    }
    None
}

fn solution(file: &str, start: char, end: char, direction: i32) -> usize {
    let (mut map, s) = parse_file(file, start, direction);
    if let Some((x, y)) = s {
        if VERBOSE >= 1 { println!("Start = ({}, {})", x, y); }
        let s = search(&mut map, x, y, end, direction).unwrap();
        if VERBOSE >= 1 {
            // for (px, py) in s.iter() {
            //     println!("{}, {}", px, py);
            // }
            let (mut map, _start) = parse_file(file, start, direction);
            for (px, py) in s.iter() {
                map[*py][*px] = '#' as u8;
            }
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    print!("{}",map[y][x] as char);
                }
                println!()
            }
            println!()
        }
        return s.len() - 1;
    } else {
        println!("Start not found");
    }
    0
}

fn main() {
    println!("Test Solution 1: {}", solution("test.txt",'S', 'E', 1));
    println!("Test Solution 2: {}", solution("test.txt", 'E', 'a', -1));

    println!("Solution 1: {}", solution("input.txt",'S', 'E', 1));
    println!("Solution 2: {}", solution("input.txt", 'E', 'a', -1));
}

#[cfg(test)]
mod test {
    use crate::{can_travel, solution};
    use more_asserts::{assert_le, assert_ge};

    #[test]
    fn test_can_travel_up() {
        assert_eq!(can_travel(1, 3, 1), false);
        assert_eq!(can_travel(2, 3, 1), true);
        assert_eq!(can_travel(3, 3, 1), true);
        assert_eq!(can_travel(4, 3, 1), true);
        assert_eq!(can_travel(5, 3, 1), true);
    }

    #[test]
    fn test_can_travel_down() {
        assert_eq!(can_travel(1, 3, -1), true);
        assert_eq!(can_travel(2, 3, -1), true);
        assert_eq!(can_travel(3, 3, -1), true);
        assert_eq!(can_travel(4, 3, -1), true);
        assert_eq!(can_travel(5, 3, -1), false);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution("test.txt", 'S', 'E', 1), 31);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution("test.txt", 'E', 'a', -1), 29);
    }

    #[test]
    fn test_real_solution() {
        assert_le!(solution("input.txt", 'S', 'E', 1), 1003);
        assert_ge!(solution("input.txt", 'S', 'E', 1), 455);
        assert_eq!(solution("input.txt", 'S', 'E', 1), 456);
    }

    #[test]
    fn test_real_solution2() {
        assert_ge!(solution("input.txt", 'E', 'a', -1), 16);
        assert_le!(solution("input.txt", 'E', 'a', -1), 456);
        assert_le!(solution("input.txt", 'E', 'a', -1), 455);
        assert_eq!(solution("input.txt", 'E', 'a', -1), 454);
    }

    #[test]
    fn test_solution_larger() {
        assert_ge!(solution("input.txt", 'E', 'a', -1), solution("input.txt", 'E', 'b', -1));
        assert_ge!(solution("input.txt", 'E', 'c', -1), solution("input.txt", 'E', 'd', -1));
        assert_ge!(solution("input.txt", 'E', 'f', -1), solution("input.txt", 'E', 'g', -1));
        assert_ge!(solution("input.txt", 'E', 'h', -1), solution("input.txt", 'E', 'i', -1));
        assert_ge!(solution("input.txt", 'E', 'k', -1), solution("input.txt", 'E', 'l', -1));
    }
}
