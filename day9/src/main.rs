use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_string(i: &str) -> (&str, u32) {
    let mut s = i.split(" ");
    (
        s.next().unwrap(),
        s.next().unwrap().parse::<u32>().unwrap()
    )
}

fn visualize(h: &Position, t: &Vec<Position>, v: &HashSet<(i32, i32)>) {
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut map = [['.'; 1000]; 1000];

    // Plot where tail have been
    for (x, y) in v.iter() {
        map[(x + map.len() as i32 / 2) as usize][(y + map.len() as i32 / 2) as usize] = '#';
    }

    // Plot where tail is now
    for (i, k) in t.iter().enumerate() {
        map[(k.x + map.len() as i32 / 2) as usize][(k.y + map.len() as i32 / 2) as usize] = char::from_digit(i as u32, 10).unwrap();
    }

    // Plot HEAD
    map[(h.x + map.len() as i32 / 2) as usize][(h.y + map.len() as i32 / 2) as usize] = 'h';

    let mut min_x = map.len();
    let mut max_x = 0;
    let mut min_y = map.len();
    let mut max_y = 0;

    for y in 0..map.len() {
        for x in 0..map.len() {
            if map[x][y] != '.' {
                min_x = min_x.min(x - 2);
                min_y = min_y.min(y - 2);
                max_x = max_x.max(x + 2);
                max_y = max_y.max(y + 2);
            }
        }
    }

    for y in min_y..max_y {
        for x in min_x..max_x {
            print!("{}", map[x][y]);
        }
        println!();
    }
}

#[derive(Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn get_direction(&self, other: &Position) -> Direction {
        let mut vy = 0;
        let mut vx = 0;

        if self.y.abs_diff(other.y) >= 2
            || self.y.abs_diff(other.y) >= 1 && self.x.abs_diff(other.x) >= 2 {
            if self.y > other.y {
                vy = 1;
            } else {
                vy = -1;
            }
        }

        if self.x.abs_diff(other.x) >= 2
            || self.x.abs_diff(other.x) >= 1 && self.y.abs_diff(other.y) >= 2 {
            if self.x > other.x {
                vx = 1;
            } else {
                vx = -1;
            }
        }

        Direction { x: vx, y: vy }
    }

    fn move_in_direction(&mut self, direction: &Direction) {
        self.x += direction.x;
        self.y += direction.y;
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Direction {
    pub x: i32,
    pub y: i32,
}

fn solution(verbose: usize, number_knots: usize) -> usize {
    let mut visited_squares = HashSet::new();
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Vec::new();
    for _ in 0..number_knots {
        tail_position.push(Position { x: 0, y: 0 });
    }
    visited_squares.insert((0, 0));

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if verbose >= 2 { println!("{}", l); }
                // parse string
                let (direction, steps) = parse_string(&l);

                // calculate move vector
                let d = match direction {
                    "R" => { Direction { x: 1, y: 0 } }
                    "L" => { Direction { x: -1, y: 0 } }
                    "U" => { Direction { x: 0, y: 1 } }
                    "D" => { Direction { x: 0, y: -1 } }
                    _ => { panic!("Invalid direction") }
                };

                // move head and tail
                (0..steps).for_each(|_| {
                    // move head
                    head_position.move_in_direction(&d);
                    let mut prev = &head_position;
                    if verbose >= 2 { println!("Head {:?}", head_position); }
                    for knot in tail_position.iter_mut() {
                        // move tail
                        knot.move_in_direction(&prev.get_direction(&knot));

                        if verbose >= 2 { println!("Knot {:?} {:?}", knot, prev); }
                        prev = knot;
                    }
                    // tag tail position
                    visited_squares.insert((prev.x, prev.y));
                });
                if verbose >= 1 { visualize(&head_position, &tail_position, &visited_squares); }
            }
        }
    }
    visited_squares.len()
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

    println!("Solution {}", solution(verbose, 1));
    println!("Solution {}", solution(verbose, 9));
}


#[cfg(test)]
mod test {
    use crate::{Direction, Position};

    #[test]
    fn test_get_direction() {
        assert_eq!(Position { x: 2, y: 1 }.get_direction(&Position { x: 1, y: 1 }), Direction { x: 0, y: 0 });
        assert_eq!(Position { x: 1, y: 2 }.get_direction(&Position { x: 1, y: 1 }), Direction { x: 0, y: 0 });
        assert_eq!(Position { x: 0, y: 1 }.get_direction(&Position { x: 1, y: 1 }), Direction { x: 0, y: 0 });
        assert_eq!(Position { x: 1, y: 0 }.get_direction(&Position { x: 1, y: 1 }), Direction { x: 0, y: 0 });

        assert_eq!(Position { x: 1, y: 1 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 0, y: 0 });
        assert_eq!(Position { x: 3, y: 3 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 0, y: 0 });
        assert_eq!(Position { x: 1, y: 3 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 0, y: 0 });
        assert_eq!(Position { x: 3, y: 1 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 0, y: 0 });

        assert_eq!(Position { x: 4, y: 3 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 1, y: 1 });
        assert_eq!(Position { x: 3, y: 4 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 1, y: 1 });

        assert_eq!(Position { x: 0, y: 1 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: -1, y: -1 });
        assert_eq!(Position { x: 0, y: 3 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: -1, y: 1 });
        assert_eq!(Position { x: 1, y: 0 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: -1, y: -1 });

        assert_eq!(Position { x: 4, y: 2 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 1, y: 0 });
        assert_eq!(Position { x: 2, y: 4 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 0, y: 1 });

        assert_eq!(Position { x: 0, y: 2 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: -1, y: 0 });
        assert_eq!(Position { x: 2, y: 0 }.get_direction(&Position { x: 2, y: 2 }), Direction { x: 0, y: -1 });

        assert_eq!(Position { x: -9, y: -16 }.get_direction(&Position { x: -11, y: -18 }), Direction { x: 1, y: 1 });
    }
}