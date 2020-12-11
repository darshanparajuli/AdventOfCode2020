use aoc_2020::*;
use std::collections::*;

fn main() {
    let input = read_input_map(|e| e.parse::<i32>().unwrap());
    part1(&input);
    part2(&input);
}

fn part1(input: &[i32]) {
    let mut input = input.iter().cloned().collect::<Vec<_>>();
    input.sort();
    let device_joltage = input.last().unwrap() + 3;

    let mut ones = 1;
    let mut threes = 1;
    let mut last = input.first().unwrap();
    for i in input.iter().skip(1) {
        let diff = i - last;
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            2 => (),
            _ => unreachable!(),
        }

        last = i;
    }

    assert!(last + 3 == device_joltage);
    let answer = ones * threes;
    println!("part 1: {}", answer);
}

/// Used hints for this one... :(
fn part2(input: &[i32]) {
    let mut input = input.iter().cloned().collect::<Vec<_>>();

    input.sort();
    input.push(*input.last().unwrap() + 3);
    input.insert(0, 0);

    let mut cache = HashMap::new();

    println!("part 2: {}", dp(&input, 0, &mut cache));
}

fn dp(input: &[i32], i: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    if i == input.len() - 1 {
        return 1;
    }

    if cache.contains_key(&i) {
        return cache[&i];
    }

    let mut count = 0;

    for k in i + 1..input.len() {
        if input[k] - input[i] <= 3 {
            count += dp(input, k, cache);
        } else {
            break;
        }
    }

    cache.insert(i, count);

    count
}
