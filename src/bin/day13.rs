use aoc_2020::{crt, read_input_map};
use std::collections::HashMap;

fn main() {
    let input: (u32, Vec<_>) = {
        let v = read_input_map(|e| e);
        let timestamp = v[0].parse::<u32>().unwrap();
        let ids = v[1]
            .split(",")
            .map(|e| {
                if e == "x" {
                    -1
                } else {
                    e.parse::<i32>().unwrap()
                }
            })
            .collect();
        (timestamp, ids)
    };
    part1(&input);
    part2(&input);
}

fn part1((timestamp, ids): &(u32, Vec<i32>)) {
    let mut depart_times = HashMap::new();

    for id in ids.iter().filter(|e| **e != -1) {
        let timestamp = *timestamp as f32;
        let tmp = (timestamp / (*id as f32)) as i32;
        let depart_time = tmp * *id;
        depart_times.insert(depart_time + id, *id);
    }

    let mut depart_times_sorted = depart_times.keys().collect::<Vec<_>>();
    depart_times_sorted.sort();

    let mut answer = 0;
    for t in depart_times_sorted {
        if (*t as u32) >= *timestamp {
            answer = *t;
            break;
        }
    }

    let answer = (answer - *timestamp as i32) * depart_times[&answer];
    println!("part 1: {}", answer);
}

fn part2((_, ids): &(u32, Vec<i32>)) {
    let mut offsets = HashMap::new();

    for (i, id) in ids.iter().enumerate() {
        if *id != -1 {
            offsets.insert(*id as u64, i as u64);
        }
    }

    let nums = ids
        .iter()
        .filter(|e| **e != -1)
        .cloned()
        .map(|e| e as u64)
        .map(|e| e.wrapping_sub(offsets[&e]))
        .collect::<Vec<u64>>();

    let mods = ids
        .iter()
        .filter(|e| **e != -1)
        .map(|e| *e as u64)
        .collect::<Vec<_>>();

    let answer = crt(&nums, &mods);
    println!("part 2: {}", answer);
}
