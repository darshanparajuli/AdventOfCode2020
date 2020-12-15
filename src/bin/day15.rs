use aoc_2020::read_input_map;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = read_input_map(|e| {
        e.split(",")
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    })
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[i32]) {
    let n = solve(input, 2020);
    println!("part 1: {}", n);
}

// Requires --release build otherwise it'll take even longer to finish.
fn part2(input: &[i32]) {
    let n = solve(input, 30000000);
    println!("part 2: {}", n);
}

fn solve(input: &[i32], nth_turn: usize) -> i32 {
    let mut spoken_count = HashMap::new();
    let mut turn_map = HashMap::new();

    for (i, n) in input.iter().enumerate() {
        spoken_count.insert(*n, 1);
        let mut v = VecDeque::new();
        v.push_back((i + 1) as i32);
        turn_map.insert(*n, v);
    }

    let mut turn = input.len();
    let mut last_num = *input.last().unwrap();

    let n = loop {
        turn += 1;
        let count = spoken_count[&last_num];
        let n = if count == 1 {
            0
        } else {
            let v = &turn_map[&last_num];
            v[v.len() - 1] - v[v.len() - 2]
        };

        if turn == nth_turn {
            break n;
        }

        *spoken_count.entry(n).or_insert(0) += 1;
        let v = turn_map.entry(n).or_insert_with(|| VecDeque::new());
        v.push_back(turn as i32);
        if v.len() > 2 {
            v.pop_front();
        }

        last_num = n;
    };

    n
}
