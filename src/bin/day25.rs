use aoc_2020::*;

fn main() {
    let input = read_input()
        .iter()
        .map(|e| e.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    part1(&input);
}

fn part1(input: &[u64]) {
    let loop_size1 = find_loop_size(7, input[0]).unwrap();
    let loop_size2 = find_loop_size(7, input[1]).unwrap();

    let encryption_key = transform(input[1], loop_size1);
    assert!(encryption_key == transform(input[0], loop_size2));
    println!("part 1: {}", encryption_key);
}

fn find_loop_size(subject_number: u64, target: u64) -> Option<u64> {
    let mut v = 1;
    for i in 0.. {
        v = (v * subject_number) % 20201227;

        if v == target {
            return Some(i + 1);
        }
    }
    None
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut v = 1;
    for _ in 0..loop_size {
        v = (v * subject_number) % 20201227;
    }
    v
}
