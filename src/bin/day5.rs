use aoc_2020::read_input;
use std::collections::HashSet;

fn main() {
    let input: Vec<_> = read_input().iter().map(|e| e.chars().collect()).collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec<char>]) {
    let max = input
        .iter()
        .map(|e| decode(e))
        .map(|(r, c)| r * 8 + c)
        .max()
        .unwrap();
    println!("part 1: {}", max);
}

fn part2(input: &[Vec<char>]) {
    let seats = input.iter().map(|e| decode(e)).collect::<Vec<_>>();

    let mut ids = HashSet::new();
    for (r, c) in &seats {
        ids.insert(r * 8 + c);
    }

    let min_row = seats.iter().map(|(r, _)| *r).min().unwrap();
    let max_row = seats.iter().map(|(r, _)| *r).max().unwrap();

    let min_col = seats.iter().map(|(_, c)| *c).min().unwrap();
    let max_col = seats.iter().map(|(_, c)| *c).max().unwrap();

    let mut result = 0;

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            let id = r * 8 + c;
            if ids.contains(&(id - 1)) && ids.contains(&(id + 1)) && !ids.contains(&id) {
                result = r * 8 + c;
                break;
            }
        }
    }

    println!("part 2: {}", result);
}

fn decode(seat: &[char]) -> (u32, u32) {
    let encoded_row = &seat[0..7];
    let mut row_min = 0;
    let mut row_max = 127;

    for k in encoded_row {
        match k {
            'F' => {
                row_max = (row_min + row_max) / 2;
            }
            'B' => {
                row_min = (row_min + row_max) / 2;
            }
            _ => unreachable!(),
        }
    }

    let encoded_col = &seat[7..];
    let mut col_min = 0;
    let mut col_max = 7;

    for k in encoded_col {
        match k {
            'L' => {
                col_max = (col_min + col_max) / 2;
            }
            'R' => {
                col_min = (col_min + col_max) / 2;
            }
            _ => unreachable!(),
        }
    }

    (row_max, col_max)
}
