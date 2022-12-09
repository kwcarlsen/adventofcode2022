use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum Command {
    ChangeDirectory(String),
    ListDirectory,
    Directory(String),
    File(String, i32),
}

fn parse_string(l: String) -> Command {
    let mut token = l.split(" ");
    return match token.next().unwrap() {
        "$" => {
            match token.next().unwrap() {
                "cd" => {
                    Command::ChangeDirectory(token.next().unwrap().to_string())
                }
                "ls" => {
                    Command::ListDirectory
                }
                _ => {
                    Command::ListDirectory
                }
            }
        }
        "dir" => {
            Command::Directory(String::from(token.next().unwrap()))
        }
        size => {
            Command::File(String::from(token.next().unwrap()), size.parse::<i32>().unwrap())
        }
    }
}

fn solution() -> i32 {
    let mut solution = 0;
    let mut directory_stack = Vec::new();
    let mut directory_size = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(l) = line {
                match parse_string(l) {
                    Command::ChangeDirectory(s) => {
                        if s == ".." {
                            println!("cd .. (pop {})", directory_size);
                            if directory_size <= 100000 {
                                solution += directory_size;
                            }
                            if directory_size > 30000000 - (70000000 - 41111105) {
                                println!("directory_size {}", directory_size);
                            }
                            directory_size += directory_stack.pop().unwrap();
                        } else {
                            println!("cd <dir> (push {})", directory_size);
                            directory_stack.push(directory_size);
                            directory_size = 0;
                        }
                    }
                    Command::File(_, size) => {
                        println!("file ({})", size);
                        directory_size += size;
                    }
                    Command::Directory(_) => {}
                    Command::ListDirectory => {}
                }
            }
        }
    }
    println!("cd .. ({})", directory_size);
    if directory_size < 100000 {
        solution += directory_size;
    }
    directory_stack.iter().for_each(|a| { println!("{}", a+directory_size)});
    solution
}

fn main() {

    println!("Solution {}", solution());
}
