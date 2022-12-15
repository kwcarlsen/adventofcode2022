use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::Element::{Digit, SubList};

static VERBOSE: u32 = 0;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Element {
    SubList(List),
    Digit(i32),
}

#[derive(Debug)]
#[derive(PartialEq)]
struct List {
    elements: Vec<Element>,
}

impl Eq for List {}

impl PartialOrd<Self> for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let c = compare_packets(self, other);
        if c == 0 {
            return Ordering::Equal;
        }
        if c > 0 {
            return Ordering::Greater;
        }
        Ordering::Less
    }

    fn max(self, other: Self) -> Self {
        panic!("Not implemented");
    }

    fn min(self, other: Self) -> Self {
        panic!("Not implemented");
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        panic!("Not implemented");
    }
}

fn parse_string(input: &str) -> (usize, List) {
    if VERBOSE >= 2 { println!("Sublist found {}", input); }
    let mut l = List { elements: Vec::new() };
    let mut buf: String = String::from("");

    let mut i = 1;
    let chars: Vec<char> = input.chars().collect();
    while i < input.len() {
        let c = chars[i];
        if VERBOSE >= 2 { println!("Parsing {}", c); }
        match c {
            '[' => {
                let (pos, sublist) = parse_string(&input[i..]);
                i += pos;
                l.elements.push(SubList(sublist));
            }
            ']' => {
                if !buf.is_empty() {
                    l.elements.push(Digit(buf.parse::<i32>().unwrap()));
                }
                return (i, l);
            }
            ',' => {
                if !buf.is_empty() {
                    l.elements.push(Digit(buf.parse::<i32>().unwrap()));
                    buf = String::from("");
                }
            }
            _ => { buf.push(c); }
        }
        i += 1;
    }
    panic!("Unexpected end of string");
}

fn is_lists_in_order(list_a: &List, list_b: &List) -> bool {
    if compare_packets(list_a, list_b) > 0 {
        return true;
    }
    false
}

fn compare_packets(list_a: &List, list_b: &List) -> i8 {
    let mut iter_b = list_b.elements.iter();
    for a in list_a.elements.iter() {
        let b = iter_b.next();
        if b.is_none() {
            return -1;
        }
        let b = b.unwrap();
        match (a, b) {
            (Digit(x), Digit(y)) => {
                if VERBOSE >= 1 { println!("Comparing {} = {}", x, y) };
                if x < y {
                    return 1;
                }
                if x > y {
                    return -1;
                }
            }
            (SubList(x), SubList(y)) => {
                let r = compare_packets(x, y);
                if r != 0 {
                    return r;
                }
            }
            (SubList(x), Digit(y)) => {
                let r = compare_packets(x, &List { elements: vec![Digit(*y)] });
                if r != 0 {
                    return r;
                }
            }
            (Digit(x), SubList(y)) => {
                let r = compare_packets(&List { elements: vec![Digit(*x)] }, y);
                if r != 0 {
                    return r;
                }
            }
        }
    }
    let b = iter_b.next();
    if b.is_none() {
        return 0;
    }
    1
}

fn solution(file: &str) -> usize {
    let mut buf = Vec::new();
    let mut s = 0;
    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            if let Ok(l) = line {
                buf.push(l);
                if i % 3 == 2 {
                    if is_lists_in_order(&parse_string(&buf[0]).1, &parse_string(&buf[1]).1) {
                        if VERBOSE >= 1 { println!("{}", (i + 1) / 3) }
                        s += (i + 1) / 3;
                    };
                    buf = Vec::new();
                }
            }
        }
    }
    s
}

fn solution2(file: &str) -> usize {
    let mut buf = BinaryHeap::new();
    buf.push(parse_string("[[2]]").1);
    buf.push(parse_string("[[6]]").1);
    let mut s = 1;
    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            if let Ok(l) = line {
                if i % 3 != 2 {
                    buf.push(parse_string(&l).1);
                }
            }
        }
    }
    for (i, x) in buf.into_sorted_vec().iter().rev().enumerate() {
        if VERBOSE >= 1 { println!("{:?}", x); }
        if x == &parse_string("[[2]]").1 ||
            x == &parse_string("[[6]]").1 {
            s *= i + 1;
        }
    }
    s
}

fn main() {
    println!("Solution part 1: {}", solution("input.txt"));
    println!("Solution part 2: {}", solution2("input.txt"));
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    use more_asserts::{assert_ge, assert_le};

    use crate::{Element::*, is_lists_in_order, List, parse_string, solution, solution2};

    #[test]
    fn test_part1_test_input() {
        assert_eq!(solution("test.txt"), 13);
    }

    #[test]
    fn test_part2_test_input() {
        assert_eq!(solution2("test.txt"), 140);
    }

    #[test]
    fn test_real_input() {
        assert_ge!(solution("input.txt"), 610+1);
        assert_ge!(solution("input.txt"), 611+1);
        assert_le!(solution("input.txt"), 5768-1);
        assert_ne!(solution("input.txt"), 3736);
        assert_ne!(solution("input.txt"), 4284);
        assert_eq!(solution("input.txt"), 5623);
    }

    #[test]
    fn test_struct() {
        let mut l = List { elements: Vec::new() };
        l.elements.push(Digit(10));

        let mut l2 = List { elements: Vec::new() };
        l2.elements.push(Digit(20));
        l2.elements.push(Digit(30));

        let mut l3 = List { elements: Vec::new() };
        l2.elements.push(SubList(l3));

        l.elements.push(SubList(l2));
        l.elements.push(Digit(100));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string("[]").1, List { elements: Vec::new() });
        assert_eq!(parse_string("[1]").1, List { elements: vec![Digit(1)] });
        assert_eq!(parse_string("[100]").1, List { elements: vec![Digit(100)] });
        assert_eq!(parse_string("[1,2]").1, List { elements: vec![Digit(1), Digit(2)] });
        assert_eq!(parse_string("[[]]").1, List { elements: vec![SubList(List { elements: vec![] })] });
        assert_eq!(parse_string("[[1]]").1, List { elements: vec![SubList(List { elements: vec![Digit(1)] })] });
        assert_eq!(parse_string("[1,[]]").1, List { elements: vec![Digit(1), SubList(List { elements: vec![] })] });
    }

    #[test]
    fn test_is_lists_in_order() {
        assert_eq!(is_lists_in_order(&parse_string("[1,1,3,1,1]").1, &parse_string("[1,1,5,1,1]").1), true);
        assert_eq!(is_lists_in_order(&parse_string("[[1],[2,3,4]]").1, &parse_string("[[1],4]").1), true);
        assert_eq!(is_lists_in_order(&parse_string("[9]").1, &parse_string("[[8,7,6]]").1), false);
        assert_eq!(is_lists_in_order(&parse_string("[[4,4],4,4]").1, &parse_string("[[4,4],4,4,4]").1), true);
        assert_eq!(is_lists_in_order(&parse_string("[7,7,7,7]").1, &parse_string("[7,7,7]").1), false);
        assert_eq!(is_lists_in_order(&parse_string("[]").1, &parse_string("[3]").1), true);
        assert_eq!(is_lists_in_order(&parse_string("[[[]]]").1, &parse_string("[[]]").1), false);
        assert_eq!(is_lists_in_order(&parse_string("[1,[2,[3,[4,[5,6,7]]]],8,9]").1, &parse_string("[1,[2,[3,[4,[5,6,0]]]],8,9]").1), false);

        assert_eq!(is_lists_in_order(&parse_string("[1,[1],2]").1, &parse_string("[1,[1],2]").1), false);
        assert_eq!(is_lists_in_order(&parse_string("[1,[1],2]").1, &parse_string("[1,[1],1]").1), false);
        assert_eq!(is_lists_in_order(&parse_string("[1,[1,1],2]").1, &parse_string("[1,[1],1]").1), false);
    }

    #[test]
    fn test_regression() {
        assert_eq!(is_lists_in_order(
            &parse_string("[[[5,[8,5]],[9,2]],[[[4,9],[3,1],[2,7,5],[2,9,2]],0],[[2,10,2,[],[4,4,4]]],[[7,[1,2,5],[],9],[8,3,[3,8,0,1,10],0,5],1,7],[[[8,0,6,2],[],4,[10]],3,[],4,8]]").1,
            &parse_string("[[[[1,8,4,2,4],[0,2,0],5],[7,[4,7,10],[]],[[4,0],2],[8,[2,0,10,4],[7,6]]],[],[1,[[1,4,9,8,6],4]]]").1,
        ), false);
    }

    #[test]
    fn test_regression_empty_list() {
        assert_eq!(is_lists_in_order(
            &parse_string("[]").1,
            &parse_string("[]").1,
        ), false);
    }

    #[test]
    fn test_regression2() {
        assert_eq!(is_lists_in_order(
            &parse_string("[[]]").1,
            &parse_string("[]").1,
        ), false);
    }

    #[test]
    fn test_regression3() {
        assert_eq!(is_lists_in_order(
            &parse_string("[]").1,
            &parse_string("[[]]").1,
        ), true);
    }

    #[test]
    fn test_regression4() {
        assert_eq!(is_lists_in_order(
            &parse_string("[1]").1,
            &parse_string("[[]]").1,
        ), false);
    }

    #[test]
    fn test_regression5() {
        assert_eq!(is_lists_in_order(
            &parse_string("[[]]").1,
            &parse_string("[1]").1,
        ), true);
    }

    #[test]
    fn test_regression6() {
        assert_eq!(is_lists_in_order(
            &parse_string("[1,2]").1,
            &parse_string("[1]").1,
        ), false);
    }

    #[test]
    fn test_regression7() {
        assert_eq!(is_lists_in_order(
            &parse_string("[1]").1,
            &parse_string("[1,2]").1,
        ), true);
    }

    #[test]
    fn test_ord() {
        assert_eq!(parse_string("[1]").1.cmp(&parse_string("[1,2]").1),
                   Ordering::Greater);
        assert_eq!(parse_string("[1,2]").1.cmp(&parse_string("[1]").1),
                   Ordering::Less);
        assert_eq!(parse_string("[1,2]").1.cmp(&parse_string("[1,2]").1),
                   Ordering::Equal);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(parse_string("[1,2,3]").1);
        heap.push(parse_string("[1]").1);
        heap.push(parse_string("[1,2]").1);

        assert_eq!(heap.pop().unwrap(), parse_string("[1]").1);
        assert_eq!(heap.pop().unwrap(), parse_string("[1,2]").1);
        assert_eq!(heap.pop().unwrap(), parse_string("[1,2,3]").1);
    }

    #[test]
    fn test_min_heap2() {
        let mut heap = BinaryHeap::new();
        heap.push(parse_string("[]").1);
        heap.push(parse_string("[[]]").1);
        heap.push(parse_string("[1,1,5,1,1]").1);
        heap.push(parse_string("[1,1,3,1,1]").1);

        assert_eq!(heap.pop().unwrap(), parse_string("[]").1);
        assert_eq!(heap.pop().unwrap(), parse_string("[[]]").1);
        assert_eq!(heap.pop().unwrap(), parse_string("[1,1,3,1,1]").1);
        assert_eq!(heap.pop().unwrap(), parse_string("[1,1,5,1,1]").1);
    }
}