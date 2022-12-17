use std::collections::{BinaryHeap, BTreeSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

static VERBOSE: u32 = 0;

struct Sensor {
    position_x: i32,
    position_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

impl Sensor {
    fn parse_line(line: &str) -> Sensor {
        lazy_static! {
        static ref RE: Regex = Regex::new(r"^Sensor at x=([\-0-9]+), y=([\-0-9]+): closest beacon is at x=([\-0-9]+), y=([\-0-9]+)$").unwrap();
    }
        if VERBOSE >= 2 { println!("{line:}"); }
        let cap = RE.captures_iter(line).next().unwrap();
        Sensor {
            position_x: cap[1].parse::<i32>().unwrap(),
            position_y: cap[2].parse::<i32>().unwrap(),
            beacon_x: cap[3].parse::<i32>().unwrap(),
            beacon_y: cap[4].parse::<i32>().unwrap(),
        }
    }

    fn radius(&self) -> i32 {
        self.position_x.abs_diff(self.beacon_x) as i32 +
            self.position_y.abs_diff(self.beacon_y) as i32
    }

    fn scan_line(&self, y: i32) -> Option<(i32, i32)> {
        let r = self.radius();
        let width = r * 2 + 1 - self.position_y.abs_diff(y) as i32 * 2;
        if width < 1 {
            return None;
        }
        let start = self.position_x - r + self.position_y.abs_diff(y) as i32;
        Some((start, width))
    }

    fn is_point_covered(&self, x: i32, y: i32) -> bool {
        let r2 = self.position_x.abs_diff(x) as i32 +
            self.position_y.abs_diff(y) as i32;

        r2 <= self.radius()
    }
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn parse_file(file: &str) -> Vec<Sensor> {
    let mut sensors = Vec::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let s = Sensor::parse_line(&line);
            sensors.push(s);
        }
    }
    sensors
}

fn solution(file: &str, line: i32) -> i32 {
    let sensors = parse_file(file);
    let mut beacons_in_scanline = BTreeSet::new();
    let mut scan_lines = BinaryHeap::new();
    for sensor in sensors {
        if let Some(s) = sensor.scan_line(line) {
            if sensor.beacon_y == line {
                if VERBOSE >= 1 { println!("Beacon in scanline {} {}", sensor.beacon_x, sensor.beacon_y); }
                beacons_in_scanline.insert((sensor.beacon_x, sensor.beacon_y));
            }
            scan_lines.push(s);
        }
    }
    let mut x = -4320198;
    let mut solution = 0;
    for (start, width) in scan_lines.into_sorted_vec() {
        let d = (x - start).clamp(0, width);
        if VERBOSE >= 1 { println!("Looking at scanline {} {} {}", start, width, d); }
        solution += width - d;
        x = x.max(start + width);
        if VERBOSE >= 1 { println!("Updated vars {} {}", x, solution); }
    }

    solution - beacons_in_scanline.len() as i32
}

fn scan_map(sensors: Vec<Sensor>, size: i32) -> (i32, i32) {
    let mut x;
    let mut y = 0;
    while y < size {
        x = 0;
        while x < size {

            let mut covered = false;
            for s in sensors.iter() {
                if s.is_point_covered(x, y) {
                    covered = true;
                    let (start, width) = s.scan_line(y).unwrap();
                    x = width + start - 1;
                }
            }
            if !covered {
                return (x, y);
            }
            x += 1;
        }
        y += 1;
    }
    (0, 0)
}

fn solution2(file: &str, size: i32) -> i64 {
    let sensors = parse_file(file);

    let (x, y) = scan_map(sensors, size);

    x as i64 * 4000000 as i64 + y as i64
}

fn main() {
    println!("Solution: {}", solution("input.txt", 2000000));
    println!("Solution: {}", solution2("input.txt", 4000000));
}

#[cfg(test)]
mod test {
    use crate::{Sensor, solution, solution2};

    #[test]
    fn test_part1_test_input() {
        assert_eq!(solution("test.txt", 10), 26);
    }

    #[test]
    fn test_part2_test_input() {
        assert_eq!(solution2("test.txt", 20), 56000011);
    }

    #[test]
    fn test_part1_input() {
        assert_ne!(solution("input.txt", 2000000), 4651344);
        assert_ne!(solution("input.txt", 2000000), 4651345);
        assert_eq!(solution("input.txt", 2000000), 5083287);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(solution2("input.txt", 4000000), 13134039205729);
    }

    #[test]
    fn test_sensor() {
        let s = Sensor {
            position_x: 8,
            position_y: 7,
            beacon_x: 2,
            beacon_y: 10,
        };

        assert_eq!(s.radius(), 9);
        assert_eq!(s.scan_line(-3), None);
        assert_eq!(s.scan_line(7), Some((-1, 19)));
        assert_eq!(s.scan_line(10), Some((2, 13)));
        assert_eq!(s.scan_line(16), Some((8, 1)));
        assert_eq!(s.scan_line(17), None);

        assert_eq!(s.is_point_covered(20, 20), false);
        assert_eq!(s.is_point_covered(17, 17), false);
        assert_eq!(s.is_point_covered(4, 4), true);
    }

}
