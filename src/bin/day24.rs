use aoc_2020::*;
use std::collections::hash_map::{Entry, HashMap};

fn main() {
    let input = read_input_map(|e| {
        let mut i = 0;
        let mut v = vec![];
        while i < e.len() {
            match &e[i..i + 1] {
                "e" | "w" => {
                    v.push(e[i..i + 1].to_owned());
                }
                _ => {
                    v.push(e[i..i + 2].to_owned());
                    i += 1;
                }
            }

            i += 1;
        }
        v
    });

    part1(input.clone());
    part2(input);
}

fn part1(input: Vec<Vec<String>>) {
    let mut floor = Floor::new();
    for row in input {
        let mut x = 0;
        let mut y = 0;
        for d in row {
            match &d[..] {
                "e" => {
                    x += 1;
                }
                "se" => {
                    x += 1;
                    y += 1;
                }
                "sw" => {
                    y += 1;
                }
                "w" => {
                    x -= 1;
                }
                "nw" => {
                    x -= 1;
                    y -= 1;
                }
                "ne" => {
                    y -= 1;
                }
                _ => unreachable!(),
            }
        }

        floor.flip(x, y);
    }

    println!("part 1: {}", floor.count_black_tiles());
}

fn part2(input: Vec<Vec<String>>) {
    let mut floor = Floor::new();
    for row in input {
        let mut x = 0;
        let mut y = 0;
        for d in row {
            match &d[..] {
                "e" => {
                    x += 1;
                }
                "se" => {
                    x += 1;
                    y += 1;
                }
                "sw" => {
                    y += 1;
                }
                "w" => {
                    x -= 1;
                }
                "nw" => {
                    x -= 1;
                    y -= 1;
                }
                "ne" => {
                    y -= 1;
                }
                _ => unreachable!(),
            }
        }

        floor.flip(x, y);
    }

    let mut to_flip = vec![];
    for _ in 0..100 {
        to_flip.clear();
        let ((min_x, max_x), (min_y, max_y)) = floor.min_max_dim();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if floor.get_color(x, y) {
                    // white tile
                    if floor.count_neighboring_black_tiles(x, y) == 2 {
                        to_flip.push((x, y));
                    }
                } else {
                    // black tile
                    let count = floor.count_neighboring_black_tiles(x, y);
                    if count == 0 || count > 2 {
                        to_flip.push((x, y));
                    }
                }
            }
        }

        for (x, y) in &to_flip {
            floor.flip(*x, *y);
        }

        // println!("day {}: {}", i + 1, floor.count_black_tiles());
    }

    println!("part 2: {}", floor.count_black_tiles());
}

struct Floor {
    // white tile = true, black tile = false
    tiles: HashMap<(i32, i32), bool>,
}

impl Floor {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    fn flip(&mut self, x: i32, y: i32) {
        match self.tiles.entry((x, y)) {
            Entry::Occupied(ref mut o) => {
                *o.get_mut() = !o.get();
            }
            Entry::Vacant(v) => {
                v.insert(false);
            }
        }
    }

    fn count_neighboring_black_tiles(&self, x: i32, y: i32) -> usize {
        let neighbors = [(1, 0), (1, 1), (0, 1), (-1, 0), (-1, -1), (0, -1)];
        let mut count = 0;

        for (dx, dy) in &neighbors {
            if let Some(v) = self.tiles.get(&(x + dx, y + dy)) {
                if !v {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_black_tiles(&self) -> usize {
        self.tiles.iter().filter(|(_, v)| !*v).count()
    }

    fn get_color(&self, x: i32, y: i32) -> bool {
        if let Some(c) = self.tiles.get(&(x, y)) {
            return *c;
        } else {
            true
        }
    }

    fn min_max_dim(&self) -> ((i32, i32), (i32, i32)) {
        let min_x = *self.tiles.keys().map(|(x, _)| x).min().unwrap() - 1;
        let max_x = *self.tiles.keys().map(|(x, _)| x).max().unwrap() + 1;
        let min_y = *self.tiles.keys().map(|(_, y)| y).min().unwrap() - 1;
        let max_y = *self.tiles.keys().map(|(_, y)| y).max().unwrap() + 1;

        ((min_x, max_x), (min_y, max_y))
    }
}
