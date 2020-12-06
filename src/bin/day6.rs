use aoc_2020::read_input;
use std::collections::HashSet;

fn main() {
    let input: Vec<_> = read_input().iter().map(|e| e.to_owned()).collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &[String]) {
    let mut count = 0;

    let mut index = 0;
    while index < input.len() {
        let mut group = HashSet::new();

        while index < input.len() {
            if input[index].is_empty() {
                break;
            }

            let s = &input[index];
            s.chars().for_each(|e| {
                group.insert(e);
            });

            index += 1;
        }

        count += group.len();
        index += 1;
    }

    println!("part 1: {}", count);
}

fn part2(input: &[String]) {
    let mut count = 0;

    let mut index = 0;
    while index < input.len() {
        let mut sets = vec![];

        while index < input.len() {
            if input[index].is_empty() {
                break;
            }

            let s = &input[index];
            sets.push(s.chars().collect::<HashSet<_>>());

            index += 1;
        }

        if sets.len() == 1 {
            count += sets.first().unwrap().len();
        } else {
            count += sets
                .windows(2)
                .map(|e| e[0].intersection(&e[1]).count())
                .min()
                .unwrap();
        }

        index += 1;
    }

    println!("part 2: {}", count);
}
