use aoc_2020::read_input;
use std::collections::HashSet;

const PREAMBLE_LEN: usize = 25;

fn main() {
    let input: Vec<_> = read_input()
        .iter()
        .map(|e| e.parse::<i64>().unwrap())
        .collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &[i64]) {
    println!("part 1: {}", get_invalid_number(input).unwrap());
}

fn part2(input: &[i64]) {
    let target = get_invalid_number(input).unwrap();

    let mut v = vec![];

    let mut i = 0;
    let mut j = 1;
    while i < input.len() && j < input.len() {
        let mut sum = 0;
        for k in i..=j {
            sum += input[k];
        }

        if sum == target {
            for k in i..=j {
                v.push(input[k]);
            }
            break;
        } else if sum < target {
            j += 1;
        } else if sum > target {
            i += 1;
        }
    }

    let min = v.iter().min().unwrap();
    let max = v.iter().max().unwrap();
    let answer = min + max;
    println!("part 1: {}", answer);
}

fn get_invalid_number(input: &[i64]) -> Option<i64> {
    let mut set = HashSet::new();
    for k in 0..PREAMBLE_LEN {
        set.insert(input[k]);
    }

    for i in PREAMBLE_LEN..input.len() {
        let current = input[i];

        let mut found = false;
        for k in i - PREAMBLE_LEN..i {
            let n = (current - input[k]).abs();
            if set.contains(&n) {
                found = true;
                break;
            }
        }

        if !found {
            return Some(current);
        }

        set.remove(&input[i - PREAMBLE_LEN]);
        set.insert(current);
    }

    None
}
