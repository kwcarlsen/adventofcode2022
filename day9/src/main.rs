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

#[derive(Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn get_direction(&self, other: &Position) -> Direction {
        let mut vy = 0;
        let mut vx = 0;

        if self.y.abs_diff(other.y) >= 2 {
            if self.y > other.y {
                vy = 1;
            } else {
                vy = -1;
            }
            if self.x.abs_diff(other.x) >= 1 {
                vx = self.x - other.x;
            }
        }
        if self.x.abs_diff(other.x) >= 2 {
            if self.x > other.x {
                vx = 1;
            } else {
                vx = -1;
            }
            if self.y.abs_diff(other.y) >= 1 {
                vy = self.y - other.y;
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

fn solution(verbose: bool) -> usize {
    let mut visited_squares = HashSet::new();
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    visited_squares.insert((tail_position.x, tail_position.y));
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if verbose { println!("{}", l); }
                // parse string
                let (direction, steps) = parse_string(&l);

                // calculate vector
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

                    // move tail
                    tail_position.move_in_direction(&head_position.get_direction(&tail_position));

                    if verbose { println!("{:?} {:?}", tail_position, head_position); }
                    // tag tail position
                    visited_squares.insert((tail_position.x, tail_position.y));
                });
            }
        }
    }
    visited_squares.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let verbose = args.iter().find(|&arg| arg == "--verbose").is_some();

    println!("Solution {}", solution(verbose));
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
    }
}