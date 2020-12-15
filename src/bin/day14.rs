use aoc_2020::read_input_map;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ProgramLine {
    Mask(String),
    Mem { address: usize, value: u64 },
}

fn main() {
    let input = read_input_map(|e| {
        let split = e.split(" = ").collect::<Vec<_>>();
        if split[0].starts_with("mask") {
            let mask = split[1].to_owned();
            ProgramLine::Mask(mask)
        } else if split[0].starts_with("mem") {
            let open_bracket = split[0].find('[').unwrap();
            let close_bracket = split[0].find(']').unwrap();
            let address = split[0][open_bracket + 1..close_bracket]
                .parse::<usize>()
                .unwrap();
            let value = split[1].parse::<u64>().unwrap();
            ProgramLine::Mem { address, value }
        } else {
            unreachable!();
        }
    });

    part1(&input);
    part2(&input);
}

fn part1(input: &[ProgramLine]) {
    let mut mem = HashMap::new();

    let mut i = 0;
    while i < input.len() {
        let line = &input[i];
        match line {
            ProgramLine::Mask(ref m) => {
                i += 1;
                while i < input.len() {
                    let line = &input[i];
                    match line {
                        ProgramLine::Mem { ref address, value } => {
                            mem.insert(address, apply_mask1(m, *value));
                        }
                        _ => {
                            break;
                        }
                    }

                    i += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    let answer = mem.values().sum::<u64>();
    println!("part 1: {}", answer);
}

fn part2(input: &[ProgramLine]) {
    let mut mem = HashMap::new();

    let mut i = 0;
    while i < input.len() {
        let line = &input[i];
        match line {
            ProgramLine::Mask(ref m) => {
                i += 1;
                while i < input.len() {
                    let line = &input[i];
                    match line {
                        ProgramLine::Mem { address, value } => {
                            let (address, floating) = apply_mask2(m, *address as u64);
                            let count = 1u64 << floating.len();
                            for i in 0..count {
                                let mut address = address;
                                for (k, f) in floating.iter().enumerate() {
                                    let k = k as u64;
                                    set_bit_at(&mut address, (i >> k) & 1u64, *f);
                                }
                                mem.insert(address, *value);
                            }
                        }
                        _ => {
                            break;
                        }
                    }

                    i += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    let answer = mem.values().sum::<u64>();
    println!("part 2: {}", answer);
}

fn apply_mask1(mask: &str, mut value: u64) -> u64 {
    for (i, c) in mask.chars().enumerate() {
        match c {
            '1' => {
                value = value | 1u64 << (mask.len() - i - 1);
            }
            '0' => {
                value = value & !(1u64 << (mask.len() - i - 1));
            }
            'X' => {}
            _ => unreachable!(),
        }
    }
    value
}

fn apply_mask2(mask: &str, mut value: u64) -> (u64, Vec<u64>) {
    let mut floating = vec![];
    for (i, c) in mask.chars().enumerate() {
        match c {
            '1' => {
                value = value | 1u64 << (mask.len() - i - 1);
            }
            '0' => {}
            'X' => {
                value = value & !(1u64 << (mask.len() - i - 1));
                floating.push((mask.len() - i - 1) as u64);
            }
            _ => unreachable!(),
        }
    }
    (value, floating)
}

fn set_bit_at(src: &mut u64, bit: u64, at: u64) {
    if bit == 0 {
        *src &= !(1u64 << at);
    } else if bit == 1 {
        *src |= bit << at;
    }
}
