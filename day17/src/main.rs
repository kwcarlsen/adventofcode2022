use std::fs;

struct Rock {
    name: &'static str,
    x: isize,
    y: isize,
    parts: Vec<(isize, isize)>,
    heights: Vec<(isize, isize)>,
}


impl Rock {
    fn line(top: isize) -> Rock {
        Rock {
            name: "line",
            x: 2,
            y: top + 4,
            parts: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            heights: vec![(0, 1), (1, 1), (2, 1), (3, 1)],
        }
    }
    fn cross(top: isize) -> Rock {
        Rock {
            name: "cross",
            x: 2,
            y: top + 4,
            parts: vec![(0, 1), (1, 1), (2, 1), (1, 0), (1, 2)],
            heights: vec![(0, 2), (1, 3), (2, 2)],
        }
    }
    fn l(top: isize) -> Rock {
        Rock {
            name: "l",
            x: 2,
            y: top + 4,
            parts: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            heights: vec![(0, 1), (1, 1), (2, 3)],
        }
    }
    fn vline(top: isize) -> Rock {
        Rock {
            name: "vline",
            x: 2,
            y: top + 4,
            parts: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            heights: vec![(0, 4)],
        }
    }
    fn boks(top: isize) -> Rock {
        Rock {
            name: "box",
            x: 2,
            y: top + 4,
            parts: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            heights: vec![(0, 2), (1, 2)],
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
        println!("Applying wind");
        let mut direction: isize;
        if wind == '<' {
            direction = -1;
        } else {
            direction = 1;
        }
        rock.x = (rock.x as isize + direction) as isize;
        if self.check_collision(&rock) {
            rock.x = (rock.x as isize - direction) as isize;
        }
        return rock;
    }

    fn apply_gravity(&mut self, mut rock: Rock) -> Option<Rock> {
        println!("Applying gravity");
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
            println!("Collision check: {} + {} = {}, {} + {} = {}", rock_x, rock.x, rock_x + rock.x, rock_y, rock.y, rock_y + rock.y);
            if self.is_tile_occupied(rock_x + rock.x, rock_y + rock.y) {
                println!("collision");
                return true;
            }
        }
        false
    }

    fn is_tile_occupied(&self, x: isize, y: isize) -> bool {
        if x < 0 ||
            x > 6 ||
            y < 0 ||
            self.bottom_line[x as usize][y as usize] == '#' {
            return true;
        }
        false
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

fn solution(file: &str) -> isize {
    let mut level = Level {
        bottom_line: vec![vec!['.'; 5000]; 7],
        rock_count: 0,
        highest_point: -1,
    };

    let data = fs::read_to_string(file).expect("could not read file");

    let mut rock = level.spawn_next_rock();

    loop {
        for wind in data.chars() {
            println!("{} {}", wind, rock.name);
            rock = level.apply_wind(rock, wind);
            match level.apply_gravity(rock) {
                None => {
                    level.print();
                    if level.rock_count == 2022 {
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


fn main() {
    println!("Hello, world! {}", solution("input.txt"));
}
