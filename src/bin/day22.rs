use aoc_2020::*;
use std::collections::*;

fn main() {
    let input = parse_input(read_input());
    part1(input.clone());
    part2(input);
}

fn part1((mut player1, mut player2): (VecDeque<u32>, VecDeque<u32>)) {
    while !player1.is_empty() && !player2.is_empty() {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();

        if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }

    let winner = if player1.is_empty() { player2 } else { player1 };

    let mut sum = 0;
    for i in 0..winner.len() {
        sum += winner[winner.len() - i - 1] * (i + 1) as u32;
    }

    println!("part 1: {}", sum);
}

fn part2((mut player1, mut player2): (VecDeque<u32>, VecDeque<u32>)) {
    let winner = match recursive_combat((&mut player1, &mut player2)) {
        1 => player1,
        2 => player2,
        _ => unreachable!(),
    };

    let mut sum = 0;
    for i in 0..winner.len() {
        sum += winner[winner.len() - i - 1] * (i + 1) as u32;
    }

    println!("part 2: {}", sum);
}

fn recursive_combat(current: (&mut VecDeque<u32>, &mut VecDeque<u32>)) -> u32 {
    let (player1, player2) = current;

    let mut prev_rounds = HashSet::new();
    prev_rounds.insert((player1.clone(), player2.clone()));
    while !player1.is_empty() && !player2.is_empty() {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();

        let round = (player1.clone(), player2.clone());
        if prev_rounds.contains(&round) {
            return 1;
        }
        prev_rounds.insert(round);

        if p1 as usize <= player1.len() && p2 as usize <= player2.len() {
            let mut a = player1.iter().copied().take(p1 as usize).collect();
            let mut b = player2.iter().copied().take(p2 as usize).collect();
            match recursive_combat((&mut a, &mut b)) {
                1 => {
                    player1.push_back(p1);
                    player1.push_back(p2);
                }
                2 => {
                    player2.push_back(p2);
                    player2.push_back(p1);
                }
                _ => unreachable!(),
            }
        } else {
            if p1 > p2 {
                player1.push_back(p1);
                player1.push_back(p2);
            } else {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        }
    }

    if player1.is_empty() {
        2
    } else {
        1
    }
}

fn parse_input(input: Vec<String>) -> (VecDeque<u32>, VecDeque<u32>) {
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();

    let mut i = 1;
    while i < input.len() {
        if input[i].is_empty() {
            break;
        }

        player1.push_back(input[i].parse::<u32>().unwrap());
        i += 1;
    }

    i += 2;
    while i < input.len() {
        player2.push_back(input[i].parse::<u32>().unwrap());
        i += 1;
    }

    (player1, player2)
}
