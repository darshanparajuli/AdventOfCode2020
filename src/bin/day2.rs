use aoc_2020::read_input;

#[derive(Debug)]
struct Password {
    min: i32,
    max: i32,
    letter: char,
    password: Vec<char>,
}

fn main() {
    let input: Vec<_> = read_input()
        .iter()
        .map(|e| e.split(" ").collect::<Vec<&str>>())
        .map(|e| {
            let (min, max) = {
                let range = e[0].split("-").collect::<Vec<_>>();
                let min = range[0].parse::<i32>().unwrap();
                let max = range[1].parse::<i32>().unwrap();
                (min, max)
            };

            let letter = e[1].chars().next().unwrap();
            Password {
                min,
                max,
                letter,
                password: e[2].chars().collect(),
            }
        })
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &[Password]) {
    fn is_valid(password: &Password) -> bool {
        let count = password
            .password
            .iter()
            .filter(|e| password.letter == **e)
            .count() as i32;
        count >= password.min && count <= password.max
    }

    let valid_count = input.iter().filter(|e| is_valid(e)).count();
    println!("part 1: {}", valid_count);
}

fn part2(input: &[Password]) {
    fn is_valid(password: &Password) -> bool {
        let mut count = 0;
        let min = password.min - 1;
        let max = password.max - 1;
        if min < password.password.len() as i32 {
            if password.password[min as usize] == password.letter {
                count += 1;
            }
        }

        if max < password.password.len() as i32 {
            if password.password[max as usize] == password.letter {
                count += 1;
            }
        }

        count == 1
    }

    let valid_count = input.iter().filter(|e| is_valid(e)).count();
    println!("part 2: {}", valid_count);
}
