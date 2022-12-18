use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

static VERBOSE: i32 = 1;

/*
     AA  ------  DD ----- EE ---- FF  ---- GG ----- HH
      0 \        20       3       0         0       22
      |  \         |
      |   \        |
     II   BB ----- CC
      0    13       2
      |
      |
     JJ
     21

     30!

     BB --2-- DD --1-- EE --3-- HH
      | \      |
      3  1    1
      |   \  /
     JJ    CC

     BB --2-- DD --1-- EE --3-- HH
      | \      |
      3  1    1
      |   \  /
     JJ    CC


     M|O
           28*20             25*13                   21*21
     MD -> OD -> MC -> MB -> OB -> MA -> MI -> MJ -> OJ -> MI
                                         13*22
     MA -> MD -> ME -> MF -> MG -> MH -> OH -> MG -> MF -> ME
     9*3              6*2
     OE -> MD -> MC -> OC

     = 20*t1 + 13*t2 + 21*t3 + 22*t4 + 3*t5 + 2*t6
       {OD, OB, OJ, OH, OE, OC} = 1651
       UpperBound {OD, ?, ?, ?, ?, ?} = t1 * {Nodes.value}
 */


struct Room {
    name: String,
    pressure: i32,
    neighbours: Vec<String>,
}

impl Room {
    fn from_string(input: &str) -> Room {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z, ]+)$").unwrap();
        }
        // let re: Regex = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z, ]+)$").unwrap();
        if VERBOSE >= 1 { println!("{}", input); }
        let cap = RE.captures_iter(input).next().unwrap();

        let mut n = Vec::new();
        let mut s = cap[3].split(", ");
        while let Some(r) = s.next() {
            n.push(String::from(r));
        }

        Room {
            name: cap[1].to_owned(),
            pressure: cap[2].parse::<i32>().unwrap(),
            neighbours: n
        }
        //
    }
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn parse_file(file: &str) -> HashMap<String, Room> {
    let mut rooms = HashMap::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let s = Room::from_string(&line);
            rooms.insert(s.name.clone(), s);
        }
    }
    rooms
}

fn calculate_distance(rooms: &HashMap<String, Room>, from: String, to: String) -> i32 {
    let mut n = VecDeque::new();
    let mut v = BTreeSet::new();
    n.push_back((0, from));

    while let Some((d, a)) = n.pop_front() {
        if a == to {
            return d;
        }
        for new_neighbour in rooms.get(&a).unwrap().neighbours.iter() {
            if let None = v.get(new_neighbour) {
                n.push_back((d+1, new_neighbour.clone()));
                v.insert(new_neighbour);
            }
        }
    }

    return -1;
}

fn calculate_pressure(rooms: &HashMap<String, Room>, valves: Vec<&String>) -> i32 {
    let mut tick = 30;
    let mut pressure = 0;
    let mut location = String::from("AA");

    for v in valves {
        let moves = calculate_distance(rooms, location, v.clone());
        if VERBOSE >= 2 { println!("You move to {} in {}", v, moves); }
        tick -= moves + 1;
        pressure += tick * rooms.get(v).unwrap().pressure;
        location = v.clone();
    }
    pressure
}

fn parse_valves(rooms: &HashMap<String, Room>) -> Vec<String> {
    let mut v = Vec::new();
    for (name, r) in rooms {
        if r.pressure > 0 {
            v.push(name.clone());
        }
    }
    v
}

fn solution(file: &str) -> i32 {
    let rooms = parse_file(file);
    let valves = parse_valves(&rooms);
    println!("Valves: {:?}", valves);

    let mut max_pressure = 0;
    let mut i = 0;
    let mut iter = valves.iter().permutations(valves.len());
    for v in iter {
        if VERBOSE >= 2 { println!("{:?}", v); }
        if VERBOSE == 1 {
            if i % 100000 == 0 {
                println!("{:?}", v);
                i =0 ;
            }
            i += 1;
        }
        max_pressure = max_pressure.max(calculate_pressure(&rooms, v));
    }

    println!("{max_pressure}");
    // for (name, room) in rooms.iter() {
    //     println!("{}", name);
    // }
    1651
}


fn main() {
    println!("Solution: {}", solution("test.txt"));
    println!("Solution: {}", solution("input.txt"));
    // println!("Solution: {}", solution("input.txt"));
}

#[cfg(test)]
mod test {
    use crate::solution;

    #[test]
    fn test_part1_test_input() {
        assert_eq!(solution("test.txt"), 1651);
    }
}