use aoc_2020::read_input;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = parse_input(read_input());
    part1(&input);
    part2(&input);
}

fn part1(input: &HashMap<u64, Vec<Vec<char>>>) {
    let mut map = HashMap::new();

    for (id, v) in input {
        map.insert(
            *id,
            get_four_sides(v).iter().cloned().collect::<HashSet<_>>(),
        );
    }

    let answer = get_four_corner_tiles(&map).iter().product::<u64>();
    println!("part 1: {}", answer);
}

fn part2(input: &HashMap<u64, Vec<Vec<char>>>) {
    let mut map = HashMap::new();

    for (id, v) in input {
        map.insert(
            *id,
            get_four_sides(v).iter().cloned().collect::<HashSet<_>>(),
        );
    }

    let top_left = *get_four_corner_tiles(&map).first().unwrap();

    let mut tiles = input
        .iter()
        .map(|(k, v)| (*k, Tile::new(*k, v.clone())))
        .collect::<HashMap<_, _>>();

    // orient the top left tile
    let mut top_left_tile = tiles[&top_left].clone();
    assert!(orient_top_left(&mut top_left_tile, &map));

    let mut row = vec![top_left_tile];
    tiles.remove(&top_left);

    let flip_functions = [
        |_: &mut Tile| {},
        |tile: &mut Tile| {
            tile.flip_x();
        },
        |tile: &mut Tile| {
            tile.flip_x();
            tile.flip_y();
        },
    ];

    // build first row
    {
        let mut tiles = tiles.clone();
        loop {
            let mut found = false;

            'outer: for v in tiles.values_mut() {
                for f in &flip_functions {
                    f(v);

                    for _ in 0..4 {
                        if row.last().unwrap().right() == v.left() {
                            row.push(v.clone());
                            found = true;
                            break 'outer;
                        }
                        v.rotate();
                    }
                }
            }

            if found {
                tiles.remove(&row.last().unwrap().id);
            } else {
                break;
            }
        }
    }

    // build first column

    let mut image = vec![row];
    {
        let mut tiles = tiles.clone();
        loop {
            let mut found = false;

            'outer2: for v in tiles.values_mut() {
                for f in &flip_functions {
                    f(v);
                    for _ in 0..4 {
                        if image.last().unwrap().first().unwrap().bottom() == v.top() {
                            image.push(vec![v.clone()]);
                            found = true;
                            break 'outer2;
                        }
                        v.rotate();
                    }
                }
            }

            if found {
                tiles.remove(&image.last().unwrap().first().unwrap().id);
            } else {
                break;
            }
        }
    }

    // combine tiles
    {
        let mut tiles = tiles.clone();

        for y in 1..image.len() {
            for x in 1..image.first().unwrap().len() {
                'outer3: for v in tiles.values_mut() {
                    for f in &flip_functions {
                        f(v);

                        for _ in 0..4 {
                            if image[y][x - 1].right() == v.left()
                                && image[y - 1][x].bottom() == v.top()
                            {
                                image[y].push(v.clone());
                                break 'outer3;
                            }
                            v.rotate();
                        }
                    }
                }

                tiles.remove(&image[y][x].id);
            }
        }
    }

    let grid_size = 10;

    // build final image
    let mut final_image = vec![];
    for y in 0..image.len() {
        for i in 1..grid_size - 1 {
            let mut v = vec![];
            for tile in image[y].iter() {
                v.extend(tile.image[i][1..grid_size - 1].to_vec());
            }
            final_image.push(v);
        }
    }

    let mut tile = Tile::new(0, final_image);

    let pattern = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .map(|e| e.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
    let total_hash_count = tile
        .image
        .iter()
        .map(|e| e.iter().filter(|e| **e == '#').count())
        .sum::<usize>() as u32;
    let pattern_hash_count = pattern
        .iter()
        .map(|e| e.iter().filter(|e| **e == '#').count())
        .sum::<usize>() as u32;
    let answer = total_hash_count - pattern_hash_count * find_monsters(&mut tile, &pattern);
    println!("part 2: {}", answer);
}

fn find_monsters(tile: &mut Tile, pattern: &[Vec<char>]) -> u32 {
    let functions = [
        |_: &mut Tile| {},
        |tile: &mut Tile| {
            tile.flip_x();
        },
        |tile: &mut Tile| {
            tile.flip_x();
            tile.flip_y();
        },
    ];

    for f in functions.iter() {
        let mut monsters = 0;
        f(tile);

        for _ in 0..4 {
            for y in 0..tile.image.len() - pattern.len() {
                for x in 0..tile.image[y].len() - pattern.first().unwrap().len() {
                    let mut found = true;
                    for m in 0..pattern.len() {
                        for n in 0..pattern[m].len() {
                            if pattern[m][n] == '#' && tile.image[y + m][x + n] != '#' {
                                found = false;
                                break;
                            }
                        }
                    }

                    if found {
                        monsters += 1;
                    }
                }
            }

            if monsters > 0 {
                return monsters;
            }

            tile.rotate();
        }
    }

    panic!("monsters not found!");
}

fn orient_top_left(top_left_tile: &mut Tile, map: &HashMap<u64, HashSet<Vec<char>>>) -> bool {
    let map = {
        let mut m = HashMap::new();
        for (k, v) in map {
            let mut set = HashSet::new();
            for i in v {
                let a = i.clone();
                let mut b = i.clone();
                b.reverse();
                set.insert(a);
                set.insert(b);
            }
            m.insert(*k, set);
        }
        m
    };

    let functions = [
        |_: &mut Tile| {},
        |tile: &mut Tile| {
            tile.flip_x();
        },
        |tile: &mut Tile| {
            tile.flip_x();
            tile.flip_y();
        },
    ];

    for f in functions.iter() {
        f(top_left_tile);

        for _ in 0..4 {
            if let Some((id, _)) = map
                .iter()
                .filter(|(k, _)| **k != top_left_tile.id)
                .find(|(_, v)| v.contains(&top_left_tile.right()))
            {
                if map
                    .iter()
                    .filter(|(k, _)| **k != top_left_tile.id && **k != *id)
                    .find(|(_, v)| v.contains(top_left_tile.bottom()))
                    .is_some()
                {
                    return true;
                }
            }
            top_left_tile.rotate();
        }
    }

    false
}

fn get_four_corner_tiles(map: &HashMap<u64, HashSet<Vec<char>>>) -> Vec<u64> {
    let mut v = vec![];
    for (current_id, four_sides) in map {
        let mut count = 0;
        for side in four_sides {
            let mut rev = side.clone();
            rev.reverse();

            count += map
                .iter()
                .filter(|(k, _)| *k != current_id)
                .find(|(_, v)| v.contains(side) || v.contains(&rev))
                .map_or_else(|| 0, |_| 1);
        }

        if count == 2 {
            v.push(*current_id);
        }
    }
    v
}

fn parse_input(input: Vec<String>) -> HashMap<u64, Vec<Vec<char>>> {
    let mut result = HashMap::new();

    let mut i = 0;
    while i < input.len() {
        if input[i].starts_with("T") {
            let v = input[i].split(" ").collect::<Vec<_>>();
            let id = v[1];
            let id = id[..id.len() - 1].parse::<u64>().unwrap();
            i += 1;

            let mut v = vec![];
            while i < input.len() {
                if input[i].is_empty() {
                    break;
                }

                v.push(input[i].chars().collect());
                i += 1;
            }

            result.insert(id, v);
        }

        i += 1;
    }

    result
}

fn get_four_sides(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut v = vec![];

    // top
    v.push(input.first().unwrap().clone());

    let mut left = vec![];
    let mut right = vec![];
    for i in 0..input.len() {
        left.push(*input[i].first().unwrap());
        right.push(*input[i].last().unwrap());
    }

    v.push(right);
    // bottom
    v.push(input.last().unwrap().clone());
    v.push(left);

    v
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Tile {
    id: u64,
    image: Vec<Vec<char>>,
}

impl Tile {
    fn new(id: u64, image: Vec<Vec<char>>) -> Self {
        Self { id, image }
    }

    fn flip_x(&mut self) {
        self.image.reverse();
    }

    fn flip_y(&mut self) {
        self.image.iter_mut().for_each(|e| e.reverse());
    }

    fn rotate(&mut self) {
        let mut image = vec![];
        for x in 0..self.image.len() {
            let mut v = vec![];
            for y in 0..self.image.len() {
                v.push(self.image[y][x]);
            }
            v.reverse();
            image.push(v);
        }
        self.image = image;
    }

    fn left(&self) -> Vec<char> {
        let mut v = vec![];
        for i in 0..self.image.len() {
            v.push(*self.image[i].first().unwrap());
        }
        v
    }

    fn right(&self) -> Vec<char> {
        let mut v = vec![];
        for i in 0..self.image.len() {
            v.push(*self.image[i].last().unwrap());
        }
        v
    }

    fn top(&self) -> &[char] {
        self.image.first().unwrap()
    }

    fn bottom(&self) -> &[char] {
        self.image.last().unwrap()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Tile: {}", self.id);
        for row in &self.image {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}
