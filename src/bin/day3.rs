use aoc_2020::read_input;

fn main() {
    let input = read_input()
        .iter()
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec<char>]) {
    println!("part 1: {}", tree_count(input, 3, 1));
}

fn part2(input: &[Vec<char>]) {
    let result = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| tree_count(input, *x, *y))
        .product::<u32>();
    println!("part 2: {}", result);
}

fn tree_count(input: &[Vec<char>], slope_x: u32, slope_y: u32) -> u32 {
    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;

    while y < input.len() {
        let row = &input[y];

        let c = row[x % row.len()];
        if c == '#' {
            tree_count += 1;
        }

        x += slope_x as usize;
        y += slope_y as usize;
    }

    return tree_count;
}
