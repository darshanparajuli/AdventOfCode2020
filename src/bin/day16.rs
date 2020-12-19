use aoc_2020::read_input_map;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Input {
    rules: HashMap<usize, ((u32, u32), (u32, u32))>,
    fields: Vec<String>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn main() {
    let input = parse_input(read_input_map(|e| e));
    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    let invalid = get_invalid_tickets(input);
    let answer = invalid.iter().flatten().sum::<u32>();
    println!("part 1: {}", answer);
}

fn part2(input: &Input) {
    let mut input = input.clone();
    let invalid = get_invalid_tickets(&input);

    // Remove invalid nearby tickets!
    {
        let mut offset = 0;
        for (i, ticket) in invalid.iter().enumerate() {
            if !ticket.is_empty() {
                input.nearby_tickets.remove(i - offset);
                offset += 1;
            }
        }
    }

    let mut map = HashMap::new();
    for i in 0..input.my_ticket.len() {
        let mut v = vec![];
        for k in 0..input.nearby_tickets.len() {
            let mut set = HashSet::new();
            for (field_index, field) in input.fields.iter().enumerate() {
                let rule = &input.rules[&field_index];
                if is_in_range(input.nearby_tickets[k][i], *rule) {
                    set.insert(field);
                }
            }
            v.push(set);
        }
        map.insert(i, v);
    }

    let mut map_intersection = HashMap::new();
    for (k, v) in &map {
        let mut set = v.first().unwrap().clone();
        for i in 1..v.len() {
            set = set.intersection(&v[i]).cloned().collect();
        }
        map_intersection.insert(*k, set);
    }

    let mut set = HashSet::new();

    loop {
        let found = map_intersection
            .iter()
            .find(|(_, v)| v.len() == 1 && !set.contains(v.iter().next().unwrap()));
        if found.is_none() {
            break;
        }

        let (found_k, v) = found.unwrap();
        let value = v.iter().next().unwrap().clone();
        let found_k = *found_k;

        for (_, v) in map_intersection.iter_mut().filter(|(k, _)| **k != found_k) {
            v.remove(&value);
        }

        set.insert(value);
    }

    let answer = map_intersection
        .iter()
        .filter(|(_, v)| v.iter().next().unwrap().starts_with("departure"))
        .map(|(k, _)| input.my_ticket[*k] as u64)
        .product::<u64>();
    println!("part 2: {}", answer);
}

fn is_in_range(value: u32, ((min_a, max_a), (min_b, max_b)): ((u32, u32), (u32, u32))) -> bool {
    (value >= min_a && value <= max_a) || (value >= min_b && value <= max_b)
}

fn get_invalid_tickets(input: &Input) -> Vec<Vec<u32>> {
    let mut invalid = vec![];
    let ranges = input.rules.values().copied().collect::<Vec<_>>();

    for row in &input.nearby_tickets {
        let mut v = vec![];
        for t in row {
            let mut valid = false;
            for r in &ranges {
                if is_in_range(*t, *r) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                v.push(*t);
            }
        }
        invalid.push(v);
    }

    invalid
}

fn parse_input(input: Vec<String>) -> Input {
    let mut rules = HashMap::new();
    let mut my_ticket = vec![];
    let mut nearby_tickets = vec![];
    let mut fields = vec![];

    let mut i = 0;
    while i < input.len() {
        match input[i].as_str() {
            "your ticket:" => {
                i += 1;
                my_ticket.extend(input[i].split(",").map(|e| e.parse::<u32>().unwrap()));
                i += 1;
            }
            "nearby tickets:" => {
                i += 1;
                while i < input.len() {
                    let v = input[i]
                        .split(",")
                        .map(|e| e.parse::<u32>().unwrap())
                        .collect::<Vec<_>>();
                    nearby_tickets.push(v);

                    i += 1;
                }
            }
            "" => {
                i += 1;
            }
            _ => {
                let v = input[i].split(":").collect::<Vec<_>>();
                let name = v[0].to_owned();
                let v = v[1].trim().split(" or ").collect::<Vec<_>>();
                let range = v[0]
                    .split("-")
                    .map(|e| e.trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                let r1 = (range[0], range[1]);
                let range = v[1]
                    .split("-")
                    .map(|e| e.trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                let r2 = (range[0], range[1]);

                fields.push(name);
                rules.insert(fields.len() - 1, (r1, r2));

                i += 1;
            }
        }
    }

    Input {
        rules,
        fields,
        my_ticket,
        nearby_tickets,
    }
}
