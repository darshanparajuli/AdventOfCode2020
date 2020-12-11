use aoc_2020::read_input_map;

fn main() {
    let input: Vec<Vec<_>> = read_input_map(|e| e.chars().collect());
    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec<char>]) {
    let mut current = input.to_vec();
    let mut next = current.clone();

    loop {
        for (y, row) in current.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    'L' => {
                        let mut count = 0;
                        for m in y as i32 - 1..=y as i32 + 1 {
                            for n in x as i32 - 1..=x as i32 + 1 {
                                if m < 0
                                    || m >= current.len() as i32
                                    || n < 0
                                    || n >= row.len() as i32
                                {
                                    count += 1;
                                    continue;
                                }
                                if m == y as i32 && n == x as i32 {
                                    continue;
                                }

                                if current[m as usize][n as usize] != '#' {
                                    count += 1;
                                }
                            }
                        }

                        if count == 8 {
                            next[y][x] = '#';
                        }
                    }
                    '#' => {
                        let mut count = 0;
                        for m in y as i32 - 1..=y as i32 + 1 {
                            for n in x as i32 - 1..=x as i32 + 1 {
                                if m < 0
                                    || m >= current.len() as i32
                                    || n < 0
                                    || n >= row.len() as i32
                                {
                                    continue;
                                }
                                if m == y as i32 && n == x as i32 {
                                    continue;
                                }

                                if current[m as usize][n as usize] == '#' {
                                    count += 1;
                                }
                            }
                        }

                        if count >= 4 {
                            next[y][x] = 'L';
                        }
                    }
                    '.' => {
                        // Floor, do nothing.
                    }
                    _ => unreachable!(),
                }
            }
        }

        if next == current {
            break;
        }

        for (y, row) in next.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                current[y][x] = next[y][x];
            }
        }
    }

    let mut count = 0;
    for row in &current {
        for c in row {
            if *c == '#' {
                count += 1;
            }
        }
    }

    println!("part 1: {}", count);
}

fn part2(input: &[Vec<char>]) {
    let mut current = input.to_vec();
    let mut next = current.clone();

    loop {
        for (y, row) in current.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    'L' => {
                        let mut count = 0;
                        for (dx, dy) in &[
                            (-1, 0),
                            (1, 0),
                            (0, -1),
                            (0, 1),
                            (-1, -1),
                            (1, 1),
                            (-1, 1),
                            (1, -1),
                        ] {
                            let mut x = x as i32 + dx;
                            let mut y = y as i32 + dy;
                            loop {
                                if y < 0
                                    || y >= current.len() as i32
                                    || x < 0
                                    || x >= row.len() as i32
                                {
                                    count += 1;
                                    break;
                                }

                                let c = current[y as usize][x as usize];
                                if c != '.' {
                                    if c == 'L' {
                                        count += 1;
                                    }
                                    break;
                                }

                                x += dx;
                                y += dy;
                            }
                        }

                        if count == 8 {
                            next[y][x] = '#';
                        }
                    }
                    '#' => {
                        let mut count = 0;
                        for (dx, dy) in &[
                            (-1, 0),
                            (1, 0),
                            (0, -1),
                            (0, 1),
                            (-1, -1),
                            (1, 1),
                            (-1, 1),
                            (1, -1),
                        ] {
                            let mut x = x as i32 + dx;
                            let mut y = y as i32 + dy;
                            loop {
                                if y < 0
                                    || y >= current.len() as i32
                                    || x < 0
                                    || x >= row.len() as i32
                                {
                                    break;
                                }

                                let c = current[y as usize][x as usize];
                                if c != '.' {
                                    if c == '#' {
                                        count += 1;
                                    }
                                    break;
                                }

                                x += dx;
                                y += dy;
                            }
                        }

                        if count >= 5 {
                            next[y][x] = 'L';
                        }
                    }
                    '.' => {
                        // Floor, do nothing.
                    }
                    _ => unreachable!(),
                }
            }
        }

        if next == current {
            break;
        }

        for (y, row) in next.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                current[y][x] = next[y][x];
            }
        }
    }

    let mut count = 0;
    for row in &current {
        for c in row {
            if *c == '#' {
                count += 1;
            }
        }
    }

    println!("part 2: {}", count);
}
