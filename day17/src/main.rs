use std::collections::{HashMap};
use std::fs;

static VERBOSE: i32 = 0;

struct Rock {
    name: &'static str,
    x: isize,
    y: isize,
    parts: Vec<(isize, isize)>,
}


impl Rock {
    fn line(top: isize) -> Rock {
        Rock {
            name: "line",
            x: 2,
            y: top + 4,
            parts: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        }
    }
    fn cross(top: isize) -> Rock {
        Rock {
            name: "cross",
            x: 2,
            y: top + 4,
            parts: vec![(0, 1), (1, 1), (2, 1), (1, 0), (1, 2)],
        }
    }
    fn l(top: isize) -> Rock {
        Rock {
            name: "l",
            x: 2,
            y: top + 4,
            parts: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        }
    }
    fn vline(top: isize) -> Rock {
        Rock {
            name: "vline",
            x: 2,
            y: top + 4,
            parts: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        }
    }
    fn boks(top: isize) -> Rock {
        Rock {
            name: "box",
            x: 2,
            y: top + 4,
            parts: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
}

struct Level {
    bottom_line: Vec<Vec<char>>,
    highest_point: isize,
    rock_count: isize,
}


impl Level {
    pub(crate) fn spawn_next_rock(&mut self) -> Rock {
        let rock;
        if self.rock_count % 5 == 0 {
            rock = Rock::line(self.highest_point);
        } else if self.rock_count % 5 == 1 {
            rock = Rock::cross(self.highest_point);
        } else if self.rock_count % 5 == 2 {
            rock = Rock::l(self.highest_point);
        } else if self.rock_count % 5 == 3 {
            rock = Rock::vline(self.highest_point);
        } else {
            rock = Rock::boks(self.highest_point);
        }
        self.rock_count += 1;
        rock
    }

    fn apply_wind(&self, mut rock: Rock, wind: char) -> Rock {
        if VERBOSE >= 2 { println!("Applying wind"); }
        let direction: isize = if wind == '<' {
            -1
        } else {
            1
        };
        rock.x += direction;
        if self.check_collision(&rock) {
            rock.x -= direction;
        }
        rock
    }

    fn apply_gravity(&mut self, mut rock: Rock) -> Option<Rock> {
        if VERBOSE >= 2{ println!("Applying gravity"); }
        rock.y -= 1;
        if self.check_collision(&rock) {
            rock.y += 1;
            for (x, y) in rock.parts {
                self.bottom_line[(rock.x + x) as usize][(rock.y + y) as usize] = '#';
                self.highest_point = self.highest_point.max(rock.y + y);
            }
            return None;
        }
        Some(rock)
    }

    fn check_collision(&self, rock: &Rock) -> bool {
        for (rock_x, rock_y) in rock.parts.iter() {
            if VERBOSE >= 2 { println!("Collision check: {} + {} = {}, {} + {} = {}", rock_x, rock.x, rock_x + rock.x, rock_y, rock.y, rock_y + rock.y); }
            if self.is_tile_occupied(rock_x + rock.x, rock_y + rock.y) {
                if VERBOSE >= 2 { println!("collision"); }
                return true;
            }
        }
        false
    }

    fn is_tile_occupied(&self, x: isize, y: isize) -> bool {
        if !(0..=6).contains(&x) ||
            y < 0 ||
            self.bottom_line[x as usize][y as usize] == '#' {
            return true;
        }
        false
    }

    fn get_highest_count(&self) -> Vec<i32> {
        let mut v = Vec::new();
        for i in self.bottom_line.iter() {
            let mut lowest = -1;
            for (h, j) in i.iter().enumerate() {
                if *j == '#' {
                    lowest = h as i32;
                }
            }
            v.push(lowest);
        }
        let pivot = v.iter().fold(i32::MAX, |carry, x| { carry.min(*x) });
        v = v.iter().map(|v| { v - pivot }).collect();
        v
    }

    fn print(&self) {
        for y in (0..30).rev() {
            for x in 0..7 {
                print!("{}", self.bottom_line[x][y])
            }
             println!();
        }
    }
}

#[derive(Debug)]
struct Cycle {
    start: isize,
    length: isize,
    height: isize
}

fn detect_cycle(file: &str, max_rock_count: isize) -> Option<Cycle> {
    let mut level = Level {
        bottom_line: vec![vec!['.'; 5000]; 7],
        rock_count: 0,
        highest_point: -1,
    };

    let data = fs::read_to_string(file).expect("could not read file");

    let mut rock = level.spawn_next_rock();
    let mut cycle_detector = HashMap::new();
    loop {
        for (w, wind) in data.chars().enumerate() {
            rock = level.apply_wind(rock, wind);
            match level.apply_gravity(rock) {
                None => {
                    let l = level.get_highest_count();
                    if let Some((p_rock_count, p_highest_point)) = cycle_detector.get(&(l.clone(), level.rock_count % 5, w)) {
                        if VERBOSE >= 1 {
                            println!("Cycle starting @ {} and repeating every {} increasing height by {} (current height: {})",
                                     p_rock_count,
                                     level.rock_count - p_rock_count,
                                     level.highest_point + 1 - p_highest_point,
                                     level.highest_point + 1
                            );
                        }
                        return Some(Cycle { start: *p_rock_count, length: level.rock_count - p_rock_count, height: level.highest_point + 1 - p_highest_point});
                    }
                    cycle_detector.insert((l, level.rock_count % 5, w), (level.rock_count, level.highest_point + 1));

                    if level.rock_count == max_rock_count {
                        return None;
                    }

                    rock = level.spawn_next_rock();
                }
                Some(r) => {
                    rock = r;
                }
            }
        }
    }
}

fn solution(file: &str, rock_count: isize) -> isize {
    let mut level = Level {
        bottom_line: vec![vec!['.'; 5000]; 7],
        rock_count: 0,
        highest_point: -1,
    };

    let data = fs::read_to_string(file).expect("could not read file");

    let mut rock = level.spawn_next_rock();

    loop {
        for wind in data.chars() {
            if VERBOSE >= 2 { println!("{} {}", wind, rock.name); }
            rock = level.apply_wind(rock, wind);
            match level.apply_gravity(rock) {
                None => {
                    if VERBOSE >= 2 { level.print(); }
                    if level.rock_count == rock_count {
                        return level.highest_point + 1;
                    }
                    rock = level.spawn_next_rock();
                }
                Some(r) => {
                    rock = r;
                }
            }
        }
    }
}

fn solution2(file: &str, stone_count: i64) -> i64 {
    let cycle = detect_cycle(file, 20000).unwrap();
    let s = solution(file, (cycle.start as i64  + ((stone_count - cycle.start as i64) % cycle.length as i64)) as isize) as i64;
    let solution = (stone_count - cycle.start as i64) / cycle.length as i64 * cycle.height as i64 + s;
    solution
}


fn main() {
    println!("Solution part1: {}", solution("input.txt", 2022));
    println!("Solution part1: {}", solution2("input.txt", 1000000000000));
}


#[cfg(test)]
mod test {
    use crate::{solution, solution2};

    #[test]
    fn test_part1_test_input() {
        assert_eq!(solution("test.txt", 2022), 3068);
    }

    #[test]
    fn test_part2_test_input() {
        assert_eq!(solution2("test.txt", 1000000000000), 1514285714288);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(solution("input.txt", 2022), 3124);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(solution2("input.txt", 1000000000000), 1561176470569);
    }
}