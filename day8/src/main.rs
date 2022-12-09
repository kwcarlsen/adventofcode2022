use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_input() -> [[u32; 99]; 99] {
    let mut map = [[0; 99]; 99];
    if let Ok(lines) = read_lines("input.txt") {
        let mut y = 0;
        for line in lines {
            if let Ok(l) = line {
                for (x, c) in l.chars().enumerate() {
                    map[x][y] = c.to_digit(10).unwrap();
                }
                y += 1;
            }
        }
    }
    map
}

fn highest_tree_in_row(map: &[[u32; 99]], x: usize, y: usize) -> u32 {
    min(
        *map[x][0..y].iter().max().unwrap(),
        *map[x][y + 1..99].iter().max().unwrap(),
    )
}

fn highest_tree_in_column(map: &[[u32; 99]], x: usize, y: usize) -> u32 {
    min(
        map[0..x].iter().fold(0, |p, row| { max(row[y], p) }),
        map[x + 1..99].iter().fold(0, |p, row| { max(row[y], p) }),
    )
}

fn highest_surrounding_tree(map: &[[u32; 99]], x: usize, y: usize) -> u32 {
    min(
        highest_tree_in_column(map, x, y),
        highest_tree_in_row(map, x, y),
    )
}

fn is_tree_covered(map: &[[u32; 99]], x: usize, y: usize) -> bool {
    highest_surrounding_tree(&map, x, y) >= map[x][y]
}

fn get_scenic_score(map: &[[u32; 99]], x: usize, y: usize) -> u32 {
    let mut n = 0;
    let mut s = 0;
    let mut w = 0;
    let mut e = 0;

    for i in (0..x).rev() {
        if map[i][y] > map[x][y] {
            break;
        }
        w = x - i;
        if map[i][y] == map[x][y] {
            break;
        }
    }

    for i in x + 1..99 {
        if map[i][y] > map[x][y] {
            break;
        }
        e = i - x;
        if map[i][y] == map[x][y] {
            break;
        }
    }

    for i in y + 1..99 {
        if map[x][i] > map[x][y] {
            break;
        }
        s = i - y;
        if map[x][i] == map[x][y] {
            break;
        }
    }
    for i in (0..y).rev() {
        if map[x][i] > map[x][y] {
            break;
        }
        n = y - i;
        if map[x][i] == map[x][y] {
            break;
        }
    }
    (w * n * s * e) as u32
}

fn solution(verbose: bool) -> (u32, u32) {
    let mut tree_count = 0;
    let mut scenic_score = 0;
    let map = read_input();
    for y in 1..98 {
        for x in 1..98 {
            if is_tree_covered(&map, x, y) {
                if verbose { print!("{}", 1) }
            } else {
                if verbose { print!("{}", 0) }
                tree_count += 1;
            }
        }
        if verbose { println!(); }
    }
    for y in 1..98 {
        for x in 1..98 {
            scenic_score = max(scenic_score, get_scenic_score(&map, x, y));
            if verbose { print!("{} ", get_scenic_score(&map, x, y)) }
        }
        if verbose { println!() }
    }
    (tree_count, scenic_score)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let verbose = args.iter().find(|&arg| arg == "--verbose").is_some();

    let (x,y) = solution(verbose);
    println!("Covered trees {}", x + 99 + 99 + 97 + 97);
    println!("Scenic score: {}", y);
}

#[cfg(test)]
mod test {
    use crate::{highest_tree_in_row, is_tree_covered, read_input};

    #[test]
    fn test_highest_tree_in_row() {
        let m = read_input();
        assert_eq!(highest_tree_in_row(&m, 1, 1), 2);
        assert_eq!(highest_tree_in_row(&m, 2, 2), 2);
        assert_eq!(highest_tree_in_row(&m, 3, 3), 2);
    }

    #[test]
    fn test_is_tree_covered() {
        let m = read_input();
        assert_eq!(is_tree_covered(&m, 1, 1), true);
        assert_eq!(is_tree_covered(&m, 4, 6), false);
        assert_eq!(is_tree_covered(&m, 4, 7), false);
        assert_eq!(is_tree_covered(&m, 37, 13), true);
        println!("{}", m[37][13]);
    }
}
