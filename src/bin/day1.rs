use aoc_2020::read_input;

fn main() {
    let input: Vec<_> = read_input()
        .iter()
        .map(|e| e.parse::<u32>().unwrap())
        .collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &[u32]) {
    let mut answer = 0;

    'outer: for i in input {
        for j in input.iter().filter(|e| *e != i) {
            if i + j == 2020 {
                answer = i * j;
                break 'outer;
            }
        }
    }

    println!("part 1: {}", answer);
}

fn part2(input: &[u32]) {
    let mut answer = 0;

    'outer: for i in input {
        for j in input.iter().filter(|e| *e != i) {
            for k in input.iter().filter(|e| *e != j) {
                if i + j + k == 2020 {
                    answer = i * j * k;
                    break 'outer;
                }
            }
        }
    }

    println!("part 2: {}", answer);
}
