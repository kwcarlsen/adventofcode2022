use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                total += match ip.as_ref() {
                    // Rock
                    "A X" => { 1 + 3 }  // draw
                    "B X" => { 1 + 0 }  // loose
                    "C X" => { 1 + 6 }  // win

                    // Paper
                    "A Y" => { 2 + 6 }  // win
                    "B Y" => { 2 + 3 }  // draw
                    "C Y" => { 2 + 0 }  // loose

                    // Scissors
                    "A Z" => { 3 + 0 }  // loose
                    "B Z" => { 3 + 6 }  // win
                    "C Z" => { 3 + 3 }  // draw
                    _ => { 0 }
                }
            }
        }
    }
    println!("{}", total);

    let mut total = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                total += match ip.as_ref() {
                    // Loose
                    "A X" => { 3 + 0 }
                    "B X" => { 1 + 0 }
                    "C X" => { 2 + 0 }

                    // Draw
                    "A Y" => { 1 + 3 }
                    "B Y" => { 2 + 3 }
                    "C Y" => { 3 + 3 }

                    // Win
                    "A Z" => { 2 + 6 }
                    "B Z" => { 3 + 6 }
                    "C Z" => { 1 + 6 }
                    _ => { 0 }
                }
            }
        }
    }
    println!("{}", total);
}