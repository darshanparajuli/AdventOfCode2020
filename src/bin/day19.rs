use aoc_2020::read_input;
use std::collections::*;

fn main() {
    let input = parse_input(read_input());
    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    let answer = input
        .messages
        .iter()
        .filter(|e| matches(e, &input.rules))
        .count();
    println!("part 1: {}", answer);
}

fn part2(input: &Input) {
    let mut input = input.clone();
    input
        .rules
        .insert(8, Rule::Multiple(vec![vec![42], vec![42, 8]]));
    input
        .rules
        .insert(11, Rule::Multiple(vec![vec![42, 31], vec![42, 11, 31]]));

    let answer = input
        .messages
        .iter()
        .filter(|e| matches2(e, &input.rules))
        .count();
    println!("part 2: {}", answer);
}

fn matches2(message: &str, rules: &HashMap<u32, Rule>) -> bool {
    for i in 0..message.len() {
        let m = &message[0..i + 1];
        if match_8(m, rules) && i < message.len() - 1 {
            let m = &message[i + 1..];
            if match_11(m, rules) {
                return true;
            }
        }
    }

    false
}

fn match_8(message: &str, rules: &HashMap<u32, Rule>) -> bool {
    let mut index = 0;
    let is_match = check_match(message, &mut index, rules, 42);
    if is_match {
        while check_match(message, &mut index, rules, 42) {}
    }
    return is_match && index == message.len();
}

fn match_11(message: &str, rules: &HashMap<u32, Rule>) -> bool {
    let mut index = 0;
    let is_match = check_match(message, &mut index, rules, 11);
    return is_match && index == message.len();
}

fn matches(message: &str, rules: &HashMap<u32, Rule>) -> bool {
    let mut index = 0;
    let is_match = check_match(message, &mut index, rules, 0);
    return is_match && index == message.len();
}

fn check_match(
    message: &str,
    index: &mut usize,
    rules: &HashMap<u32, Rule>,
    rule_index: u32,
) -> bool {
    return match &rules[&rule_index] {
        Rule::Char(s) => {
            let i = *index;
            *index += 1;
            if i < message.len() {
                return &message[i..i + 1] == s;
            } else {
                return false;
            }
        }
        Rule::Multiple(v) => {
            let mut match_found = false;
            for r in v {
                let mut i = *index;
                if r.iter().all(|e| check_match(message, &mut i, rules, *e)) {
                    *index = i;
                    match_found = true;
                    break;
                }
            }
            match_found
        }
    };
}

#[derive(Debug, Clone)]
enum Rule {
    Char(String),
    Multiple(Vec<Vec<u32>>),
}

#[derive(Debug, Clone)]
struct Input {
    rules: HashMap<u32, Rule>,
    messages: Vec<String>,
}

fn parse_input(input: Vec<String>) -> Input {
    let pos = input.iter().position(|e| &e[..] == "").unwrap();
    let messages = input[pos + 1..].to_vec();
    let rules = input[..pos]
        .iter()
        .map(|e| {
            let v = e.split(": ").collect::<Vec<_>>();
            let index = v[0].parse::<u32>().unwrap();
            let rule = v[1];

            let rule = if rule.contains("|") {
                Rule::Multiple(
                    rule.split(" | ")
                        .map(|e| {
                            e.split(" ")
                                .map(|e| e.parse::<u32>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                )
            } else if rule.contains("\"") {
                Rule::Char(rule[1..rule.len() - 1].to_owned())
            } else {
                Rule::Multiple(vec![rule
                    .split(" ")
                    .map(|e| e.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()])
            };

            (index, rule)
        })
        .collect::<HashMap<_, _>>();
    Input { rules, messages }
}
