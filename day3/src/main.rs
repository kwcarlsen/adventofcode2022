use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn split_string(input: &str) -> (&str, &str) {
    input.split_at(input.len() / 2)
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Item {
    item: char,
}

impl Item {
    fn new(item: char) -> Item {
        Item { item }
    }

    fn get_prio(&self) -> isize {
        if self.item as isize > 96 {
            return self.item as isize - 96;
        }
        self.item as isize - 64 + 26
    }
}

struct Rucksack {
    items: HashSet<Item>,
}

impl Rucksack {
    fn new() -> Rucksack {
        Rucksack { items: HashSet::new() }
    }

    fn create_from_string(input: &str) -> Rucksack {
        let mut r = Rucksack::new();
        input.chars().for_each(|i| {
            r.add_item(Item::new(i))
        });
        r
    }

    fn add_item(&mut self, i: Item) {
        self.items.insert(i);
    }

    fn remove_item(&mut self, i: &Item) {
        self.items.remove(i);
    }

    fn contains(&self, i: &Item) -> bool {
        self.items.contains(i)
    }

    fn get_items(&self) -> Vec<Item> {
        let mut v: Vec<Item> = Vec::new();
        for i in self.items.iter() {
            v.push(i.clone());
        }
        v
    }

    fn contains_list(&self, items: Vec<Item>) -> Vec<Item> {
        items.into_iter().filter(|item| { self.contains(item) }).collect()
    }
}

struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    fn new() -> Self {
        Group { rucksacks: Vec::new() }
    }

    fn get_common_item(&self) -> Vec<Item> {
        let mut items = self.rucksacks.get(0).unwrap().get_items();
        for other_rucksack in self.rucksacks[1..].iter() {
            items = other_rucksack.contains_list(items);
        }
        items
    }

    fn add_rucksack(&mut self, rucksack: Rucksack) {
        self.rucksacks.push(rucksack);
    }
}

fn evaluate_string(input: &str) -> isize {
    let (r1, r2) = split_string(input);
    let mut rucksack = Rucksack::new();
    for i in r1.chars() {
        rucksack.add_item(Item::new(i));
    }
    let mut total = 0;
    for i in r2.chars() {
        let item = Item::new(i);
        if rucksack.contains(&item) {
            total += item.get_prio();
            rucksack.remove_item(&item);
        }
    }
    total
}

fn main() {
    let mut total: isize = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                total += evaluate_string(ip.as_ref());
            }
        }
    }
    println!("{}", total);

    let mut total: isize = 0;
    let mut group = Group::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.enumerate() {
            if let (i, Ok(ip)) = line {
                group.add_rucksack(Rucksack::create_from_string(&ip));
                if i % 3 == 2 {
                    total += group.get_common_item().iter().fold(0, |v, i| { v + i.get_prio() });
                    group = Group::new();
                }
            }
        }
    }
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use crate::{evaluate_string, Group, Item, Rucksack, split_string};

    #[test]
    fn test_parse_string() {
        assert_eq!(split_string(&"vJrwpWtwJgWrhcsFMMfFFhFp"), ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
        assert_eq!(split_string(&"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"));
        assert_eq!(split_string(&"PmmdzqPrVvPwwTWBwg"), ("PmmdzqPrV", "vPwwTWBwg"));
    }

    #[test]
    fn test_get_prio() {
        assert_eq!(Item::new('p').get_prio(), 16);
        assert_eq!(Item::new('L').get_prio(), 38);
        assert_eq!(Item::new('P').get_prio(), 42);
        assert_eq!(Item::new('v').get_prio(), 22);
        assert_eq!(Item::new('t').get_prio(), 20);
        assert_eq!(Item::new('s').get_prio(), 19);
    }

    #[test]
    fn test_evaluate_string() {
        assert_eq!(evaluate_string("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(evaluate_string("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
        assert_eq!(evaluate_string("PmmdzqPrVvPwwTWBwg"), 42);
        assert_eq!(evaluate_string("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
        assert_eq!(evaluate_string("ttgJtRGJQctTZtZT"), 20);
        assert_eq!(evaluate_string("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
    }

    #[test]
    fn test_group() {
        let mut g = Group::new();
        g.add_rucksack(Rucksack::create_from_string("vJrwpWtwJgWrhcsFMMfFFhFp"));
        g.add_rucksack(Rucksack::create_from_string("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        g.add_rucksack(Rucksack::create_from_string("PmmdzqPrVvPwwTWBwg"));

        assert_eq!(g.get_common_item(), vec![Item::new('r')]);

        let mut g = Group::new();
        g.add_rucksack(Rucksack::create_from_string("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
        g.add_rucksack(Rucksack::create_from_string("ttgJtRGJQctTZtZT"));
        g.add_rucksack(Rucksack::create_from_string("CrZsJsPPZsGzwwsLwLmpwMDw"));

        assert_eq!(g.get_common_item(), vec![Item::new('Z')]);
    }
}