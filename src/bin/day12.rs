use aoc_2020::read_input_map;

fn main() {
    let input: Vec<_> =
        read_input_map(|e| (e.chars().next().unwrap(), e[1..].parse::<i32>().unwrap()));
    part1(&input);
    part2(&input);
}

fn part1(input: &[(char, i32)]) {
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;

    for (action, value) in input {
        match action {
            'N' => {
                y -= value;
            }
            'S' => {
                y += value;
            }
            'E' => {
                x += value;
            }
            'W' => {
                x -= value;
            }
            'L' => {
                let count = value / 90;
                dir = (dir + count) % 4;
            }
            'R' => {
                let count = value / 90;
                dir = ((dir - count) + 4) % 4;
            }
            'F' => match dir {
                0 => {
                    x += value;
                }
                1 => {
                    y -= value;
                }
                2 => {
                    x -= value;
                }
                3 => {
                    y += value;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    let answer = x.abs() + y.abs();
    println!("part 1: {}", answer);
}

fn part2(input: &[(char, i32)]) {
    let mut x = 0;
    let mut y = 0;

    let mut wx = 10;
    let mut wy = -1;

    for (action, value) in input {
        match action {
            'N' => {
                wy -= value;
            }
            'S' => {
                wy += value;
            }
            'E' => {
                wx += value;
            }
            'W' => {
                wx -= value;
            }
            'L' => match value {
                90 => {
                    // 10, -1 => -1, -10
                    let tmp = wx;
                    wx = wy;
                    wy = -tmp;
                }
                180 => {
                    // 10, -1 => -10, 1
                    wx = -wx;
                    wy = -wy;
                }
                270 => {
                    // 10, -1 => 1, 10
                    let tmp = wx;
                    wx = -wy;
                    wy = tmp;
                }
                _ => unreachable!(),
            },
            'R' => match value {
                90 => {
                    // 10, -1 => -10, 1
                    let tmp = wx;
                    wx = -wy;
                    wy = tmp;
                }
                180 => {
                    // 10, -1 => 1, -10
                    wx = -wx;
                    wy = -wy;
                }
                270 => {
                    // 10, -1 => -1, 10
                    let tmp = wx;
                    wx = wy;
                    wy = -tmp;
                }

                _ => unreachable!(),
            },
            'F' => {
                x += wx * value;
                y += wy * value;
            }
            _ => unreachable!(),
        }
    }

    let answer = x.abs() + y.abs();
    println!("part 2: {}", answer);
}
