use aoc_2020::read_input;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input: HashMap<_, _> = read_input()
        .iter()
        .map(|e| {
            let index = e.find("contain").unwrap();
            let bag_color = e[..index - 6].to_owned();

            let mut can_contain = HashMap::new();
            let e = &e[index + 8..];

            if !e.contains("no") {
                e.split(",").map(|e| e.trim()).for_each(|e| {
                    let tokens = e.split(" ").collect::<Vec<_>>();
                    let count = tokens[0].parse::<u32>().unwrap();
                    let color = tokens[1].to_owned() + " " + tokens[2];
                    can_contain.insert(color, count);
                });
            }

            (bag_color, can_contain)
        })
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &HashMap<String, HashMap<String, u32>>) {
    let target = "shiny gold";

    let mut count = 0;
    let mut queue = VecDeque::new();

    for (bag, _) in input {
        queue.clear();
        queue.push_back(bag);

        while let Some(bag) = queue.pop_front() {
            let contains = &input[bag];
            if contains.contains_key(target) {
                count += 1;
                break;
            } else {
                contains.keys().for_each(|e| queue.push_back(e));
            }
        }
    }

    println!("part 1: {}", count);
}

fn part2(input: &HashMap<String, HashMap<String, u32>>) {
    let target = "shiny gold";

    let mut count = 0;

    let mut queue = VecDeque::new();
    queue.push_back((target, 1));

    while let Some((bag, multiplier)) = queue.pop_front() {
        let contains = &input[bag];
        for (k, v) in contains {
            queue.push_back((k, v * multiplier));
            count += v * multiplier;
        }
    }

    println!("part 2: {}", count);
}
