use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops;
use std::time::Instant;

use itertools::Itertools;

type Bags = HashMap<String, BagContents>;
type Cache<'a> = HashMap<&'a str, bool>;

#[derive(Debug)]
struct BagContents {
    data: String,
    length: usize,
}

impl BagContents {
    fn new(line: String) -> (String, Self) {
        let (name, bags_str) = line.split("contain").collect_tuple().unwrap();

        let length = match bags_str.contains("no other bag") {
            true => 0,
            false => bags_str.split(",").count(),
        };
        let name = name.split(" bag").nth(0).unwrap().to_string();
        let contents = BagContents {
            data: bags_str.to_string(),
            length,
        };

        (name, contents)
    }
}

impl ops::Index<usize> for BagContents {
    type Output = str;

    fn index<'a>(&'a self, index: usize) -> &'a str {
        if index >= self.length {
            panic!("Index {} is out of bounds", index);
        }

        let bag_str = self.data.split(",").nth(index).unwrap().trim();
        let (_, bag_str) = bag_str.split_at(2);

        bag_str.split(" bag").nth(0).unwrap().trim()
    }
}

fn get_bags() -> Bags {
    let file = File::open("../input2.txt").expect("Unable to open file");
    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| BagContents::new(line))
        .collect()
}

fn can_contain<'a, 'b>(cache: &'b mut Cache<'a>, bags: &'a Bags, bag: &'a str) -> bool {
    if cache.contains_key(bag) {
        return cache[bag];
    }

    let contents = &bags[bag];
    let result = (0..contents.length).any(|index| {
        let inner_bag = &contents[index];
        inner_bag == "shiny gold" || can_contain(cache, bags, inner_bag)
    });

    cache.insert(bag, result);
    result
}

fn part_1(bags: &Bags) {
    let mut cache: Cache = HashMap::new();
    let count = bags
        .iter()
        .filter(|entry| can_contain(&mut cache, bags, entry.0))
        .count();

    println!("{} bags can contain shiny gold bag", count);
}

fn main() {
    println!("Part 1: \n----------");

    let start = Instant::now();
    let bags = get_bags();
    part_1(&bags);
    println!("Time Taken: {:?}", start.elapsed());
}
