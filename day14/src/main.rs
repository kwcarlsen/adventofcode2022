use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Map = Vec<Vec<char>>;
type Point = (usize, usize);
type PolyLine = Vec<Point>;

static VERBOSE: u32 = 0;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_string(input: &str) -> PolyLine {
    let mut poly_line = PolyLine::new();
    let mut poly_line_tokens = input.split(" -> ");
    for point in poly_line_tokens {
        let mut point_tokens = point.split(",");
        let x = point_tokens.next().unwrap().parse::<usize>().unwrap();
        let y = point_tokens.next().unwrap().parse::<usize>().unwrap();
        poly_line.push((x, y));
    }
    poly_line
}

fn map_is_defind(x: usize, y: usize, mut map: Map) -> Map {
    if map.len() <= y {
        for _ in map.len()..y + 1 {
            map.push(Vec::new());
        }
    }

    if map[y].len() <= x {
        for _ in map[y].len()..x + 1 {
            map[y].push('.');
        }
    }

    map
}

fn draw_line(from: Point, to: Point, mut map: Map) -> Map {
    let (mut x1, mut y1) = from;
    let (mut x2, mut y2) = to;
    if x1 > x2 {
        let x = x1;
        x1 = x2;
        x2 = x;
    }
    if y1 > y2 {
        let y = y1;
        y1 = y2;
        y2 = y;
    }

    if x1 < x2 {
        for x in x1..x2 + 1 {
            map = map_is_defind(x, y1, map);
            map[y1][x] = '#';
        }
    }
    if y1 < y2 {
        for y in y1..y2 + 1 {
            map = map_is_defind(x1, y, map);
            map[y][x1] = '#';
        }
    }

    map
}

fn draw_poly_line(mut poly_line: PolyLine, mut map: Map) -> Map {
    let mut from = None;
    for to in poly_line {
        if let Some(f) = from {
            map = draw_line(f, to, map);
            // map = draw_map(map);
            // println!();
        }
        from = Some(to);
    }
    map
}

fn draw_map(mut map: Map) -> Map {
    let mut min_x = 9999;
    let mut max_x = 0;
    let mut min_y = 9999;
    let mut max_y = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map = map_is_defind(x, y, map);
            if map[y][x] != '.' && x > 2 && y > 2 {
                min_x = min_x.min(x - 2);
                min_y = min_y.min(y - 2);
                max_x = max_x.max(x + 2);
                max_y = max_y.max(y + 2);
            }
        }
    }

    for y in min_y..max_y {
        for x in min_x..max_x {
            map = map_is_defind(x, y, map);
            print!("{}", map[y][x]);
        }
        println!();
    }
    map
}

fn drop_sand(x: usize, y: usize, mut map: Map) -> (Option<Point>, Map) {
    if x > 1000 || y > 1000 {
        return (None, map);
    }

    map = map_is_defind(x, y, map);
    map = map_is_defind(x, y + 1, map);
    map = map_is_defind(x - 1, y + 1, map);
    map = map_is_defind(x + 1, y + 1, map);

    if map[y][x] == 'o' {
        return (None, map);
    }

    if map[y + 1][x] == '.' {
        return drop_sand(x, y + 1, map);
    } else if map[y + 1][x - 1] == '.' {
        return drop_sand(x - 1, y + 1, map);
    } else if map[y + 1][x + 1] == '.' {
        return drop_sand(x + 1, y + 1, map);
    }
    map[y][x] = 'o';

    (Some((x, y)), map)
}

fn parse_file(file: &str) -> Map {
    let mut map: Map = Vec::new();
    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            if let Ok(l) = line {
                let poly_line = parse_string(&l);
                map = draw_poly_line(poly_line, map);
            }
        }
    }
    map
}

fn solution(file: &str) -> i32 {
    let mut map = parse_file(file);
    let mut solution = 0;
    while let (Some(_), new_map) = drop_sand(500, 0, map) {
        map = new_map;
        // map = draw_map(map);
        solution += 1;
    }
    solution
}

fn solution2(file: &str) -> i32 {
    let mut map = parse_file(file);
    map = draw_line((200, map.len() + 1), (800, map.len() + 1), map);

    let mut solution = 0;

    let mut dropped_sand = Some((1, 1));
    while dropped_sand.is_some() {
        (dropped_sand, map) = drop_sand(500, 0, map);
        solution += 1;
    }
    map = draw_map(map);
    solution - 1
}

fn main() {
    println!("Solution part1: {}", solution("input.txt"));
    println!("Solution part2: {}", solution2("input.txt"));
}

#[cfg(test)]
mod test {
    use crate::{solution, solution2};

    #[test]
    fn test_part1_test_input() {
        assert_eq!(solution("test.txt"), 24);
    }

    #[test]
    fn test_part2_test_input() {
        assert_eq!(solution2("test.txt"), 93);
    }

   #[test]
    fn test_part1_input() {
        assert_eq!(solution("input.txt"), 913);
    }

    #[test]
    fn test_part2_input() {
        assert_ne!(solution2("input.txt"), 8168);
        assert_eq!(solution2("input.txt"), 30762);
    }
}