use aoc_2020::read_input_map;
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

/// Solved using Chinese Remainder Theorem.
/// Learned something new!
fn part2((_, ids): &(u32, Vec<i32>)) {
    let mut offsets = HashMap::new();

    for (i, id) in ids.iter().enumerate() {
        if *id != -1 {
            offsets.insert(*id as u64, i as u64);
        }
    }

    let ids = ids
        .iter()
        .filter(|e| **e != -1)
        .cloned()
        .map(|e| e as u64)
        .collect::<Vec<u64>>();

    // (x, n) =>  x mod n
    let mods = ids
        .iter()
        .map(|e| (e - offsets[e], *e))
        .collect::<Vec<(u64, u64)>>();

    let bi = mods.iter().map(|(e, _)| *e).collect::<Vec<_>>();
    let n = mods.iter().map(|(_, m)| *m).product::<u64>();
    let ni = mods.iter().map(|(_, m)| n / m).collect::<Vec<_>>();

    let xi = ni
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let (_, m) = mods[i];

            let mut count = 1;
            while (n * count) % m != 1 {
                count += 1;
            }

            count
        })
        .collect::<Vec<_>>();

    let bi_ni_xi = (0..mods.len())
        .map(|i| bi[i] * ni[i] * xi[i])
        .collect::<Vec<_>>();

    let bi_ni_xi_sum = bi_ni_xi.iter().sum::<u64>();
    let answer = bi_ni_xi_sum % n;

    println!("part 2: {}", answer);
}
