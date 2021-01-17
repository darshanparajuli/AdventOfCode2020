use aoc_2020::*;
use std::collections::BTreeMap;

fn main() {
    let input = read_input()
        .first()
        .unwrap()
        .chars()
        .map(|e| e.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    part1(input.clone());
    part2(input);
}

fn part1(input: Vec<i32>) {
    let mut crab_cups = CrabCups::new(&input);
    let mut i = 0;
    let mut tmp = vec![];
    for _ in 0..100 {
        let picked = crab_cups.get(i);

        tmp.clear();
        for _ in 0..3 {
            let a = (i + 1) % crab_cups.count;
            tmp.push(crab_cups.take(a));
        }

        let destination = {
            let mut picked = picked;
            'outer: loop {
                if picked == 0 {
                    let (k, _) = crab_cups.max();
                    break k;
                } else {
                    picked -= 1;
                    for k in 0..crab_cups.count - 1 {
                        let next = (i + k + 1) % crab_cups.count;
                        if crab_cups.contains(next) && crab_cups.get(next) == picked {
                            break 'outer next;
                        }
                    }
                }
            }
        };

        for i in tmp.iter().rev() {
            crab_cups.put((destination + 1) % crab_cups.count, *i);
        }

        i = (i + 1) % crab_cups.count;
    }

    let (index_1, _) = crab_cups.map.iter().find(|(_, v)| **v == 1).unwrap();
    let mut s = String::new();
    for i in 1..crab_cups.count {
        let k = (index_1 + i) % crab_cups.count;
        s.push_str(&format!("{}", crab_cups.get(k)));
    }
    println!("part 1: {}", s);
}

fn part2(input: Vec<i32>) {
    let mut v = input.to_vec();
    let max = v.iter().max().unwrap();
    for i in max + 1..=1_000_000 {
        v.push(i);
    }

    let mut crab_cups = CrabCups::new(&v);
    let mut i = 0;
    let mut tmp = vec![];
    for _ in 0..10_000_000 {
        let picked = crab_cups.get(i);

        tmp.clear();
        for _ in 0..3 {
            let a = (i + 1) % crab_cups.count;
            tmp.push(crab_cups.take(a));
        }

        let destination = {
            let mut picked = picked;
            'outer: loop {
                if picked == 0 {
                    let (k, _) = crab_cups.max();
                    break k;
                } else {
                    picked -= 1;
                    for k in 0..crab_cups.count - 1 {
                        let next = (i + k + 1) % crab_cups.count;
                        if crab_cups.contains(next) && crab_cups.get(next) == picked {
                            break 'outer next;
                        }
                    }
                }
            }
        };

        for i in tmp.iter().rev() {
            crab_cups.put((destination + 1) % crab_cups.count, *i);
        }

        i = (i + 1) % crab_cups.count;

        let (index_1, _) = crab_cups.map.iter().find(|(_, v)| **v == 1).unwrap();
        for i in 0..3 {
            let k = (index_1 + i) % crab_cups.count;
            print!("{} ", crab_cups.get(k));
        }
        println!();
    }

    let (index_1, _) = crab_cups.map.iter().find(|(_, v)| **v == 1).unwrap();
    let mut s = String::new();
    for i in 1..crab_cups.count {
        let k = (index_1 + i) % crab_cups.count;
        s.push_str(&format!("{}", crab_cups.get(k)));
    }

    println!("part 2: {}", s);
}

struct CrabCups {
    count: i32,
    map: BTreeMap<i32, i32>,
}

impl CrabCups {
    fn new(input: &[i32]) -> Self {
        Self {
            count: input.len() as i32,
            map: input
                .iter()
                .enumerate()
                .map(|(i, e)| (i as i32, *e))
                .collect(),
        }
    }

    fn contains(&self, index: i32) -> bool {
        self.map.contains_key(&index)
    }

    fn get(&self, index: i32) -> i32 {
        self.map[&index]
    }

    fn take(&mut self, index: i32) -> i32 {
        let value = self.map.remove(&index).unwrap();
        let mut i = index;

        for _ in 0..self.count - 1 {
            let j = (i + 1) % self.count;
            let a = self.map.remove(&j);
            if a.is_some() {
                self.map.insert(i, a.unwrap());
            } else {
                self.map.remove(&i);
            }
            i = (i + 1) % self.count;
        }

        value
    }

    fn put(&mut self, index: i32, value: i32) {
        let mut last = None;
        let mut i = index % self.count;

        for _ in 0..self.count - 1 {
            let at = i % self.count;

            let a = self.map.remove(&at);
            if let Some(last) = last {
                self.map.insert(at, last);
            }

            if a.is_none() {
                break;
            }

            last = a;
            i = (i + 1) % self.count;
        }

        self.map.insert(index, value);
    }

    #[allow(dead_code)]
    fn to_vec(&self) -> Vec<i32> {
        self.map.iter().map(|(_, v)| v).copied().collect::<Vec<_>>()
    }

    fn max(&self) -> (i32, i32) {
        self.map
            .iter()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(k, v)| (*k, *v))
            .unwrap()
    }
}
