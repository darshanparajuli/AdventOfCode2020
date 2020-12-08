use aoc_2020::read_input;
use std::collections::HashSet;

fn main() {
    let input: Vec<_> = read_input()
        .iter()
        .map(|e| {
            let tokens = e.split(" ").collect::<Vec<_>>();
            let value = tokens[1].parse::<i32>().unwrap();
            match tokens[0] {
                "jmp" => Instruction::JMP(value),
                "acc" => Instruction::ACC(value),
                "nop" => Instruction::NOP(value),
                _ => unreachable!(),
            }
        })
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &[Instruction]) {
    let mut ptr = 0i32;
    let mut global_value = 0;
    let mut prev = HashSet::new();

    loop {
        let instruction = &input[ptr as usize];
        if prev.contains(&ptr) {
            break;
        } else {
            prev.insert(ptr);
        }

        match instruction {
            Instruction::ACC(value) => {
                global_value += value;
                ptr += 1;
            }
            Instruction::JMP(value) => {
                ptr += value;
            }
            Instruction::NOP(_) => {
                ptr += 1;
            }
        }
    }

    println!("part 1: {}", global_value);
}

fn has_infinite_loop(input: &[Instruction]) -> bool {
    let mut ptr = 0i32;
    let mut prev = HashSet::new();

    while ptr < input.len() as i32 {
        let instruction = &input[ptr as usize];
        if prev.contains(&ptr) {
            return true;
        } else {
            prev.insert(ptr);
        }

        match instruction {
            Instruction::ACC(_) => {
                ptr += 1;
            }
            Instruction::JMP(value) => {
                ptr += value;
            }
            Instruction::NOP(_) => {
                ptr += 1;
            }
        }
    }

    false
}

fn part2(input: &[Instruction]) {
    let mut input = input.to_vec();
    let mut global_value = 0;

    let mut last_jump_ptr = input.len();
    loop {
        last_jump_ptr = input[..last_jump_ptr]
            .iter()
            .rposition(|e| match e {
                Instruction::JMP(_) => true,
                _ => false,
            })
            .unwrap();

        let old_instruction = input[last_jump_ptr];
        input[last_jump_ptr] = Instruction::NOP(0);

        if !has_infinite_loop(&input) {
            break;
        } else {
            input[last_jump_ptr] = old_instruction;
        }
    }

    let mut ptr = 0i32;
    while ptr < input.len() as i32 {
        let instruction = &input[ptr as usize];
        match instruction {
            Instruction::ACC(value) => {
                global_value += value;
                ptr += 1;
            }
            Instruction::JMP(value) => {
                ptr += value;
            }
            Instruction::NOP(_) => {
                ptr += 1;
            }
        }
    }

    println!("part 2: {}", global_value);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}
