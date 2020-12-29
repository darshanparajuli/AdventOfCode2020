use aoc_2020::*;
use std::collections::*;

#[derive(Debug, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn main() {
    let input = read_input_map(|e| {
        let size = e.len();
        let v = e[0..size - 1].split(" (contains ").collect::<Vec<_>>();
        let ingredients = v[0].split(" ").map(|e| e.to_owned()).collect::<Vec<_>>();
        let allergens = v[1].split(", ").map(|e| e.to_owned()).collect::<Vec<_>>();
        Food {
            ingredients,
            allergens,
        }
    });

    part1(&input);
    part2(&input);
}

fn part1(input: &[Food]) {
    let mut ingredients = input
        .iter()
        .map(|e| {
            e.ingredients
                .iter()
                .map(|e| e.to_owned())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();
    let mut allergens = input
        .iter()
        .map(|e| {
            e.allergens
                .iter()
                .map(|e| e.to_owned())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    while allergens.iter().map(|e| e.len()).sum::<usize>() > 0 {
        for i in 0..input.len() {
            if allergens[i].is_empty() {
                continue;
            }

            let mut v = vec![];
            let mut count_map = HashMap::new();
            let mut count = 0;
            for ingr in &ingredients[i] {
                for a in &allergens[i] {
                    count_map.insert((ingr.to_owned(), a.to_owned()), 0);
                }

                for j in 0..input.len() {
                    if j == i {
                        continue;
                    }

                    if ingredients[j].contains(ingr) {
                        count += 1;
                        for a in &allergens[i] {
                            if allergens[j].contains(a) {
                                *count_map
                                    .entry((ingr.to_owned(), a.to_owned()))
                                    .or_insert(0) += 1;
                            }
                        }
                    }
                }
            }

            let ((ingr, aller), c) = count_map
                .iter()
                .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
                .unwrap();
            if *c > 0
                && count_map
                    .iter()
                    .filter(|(k, _)| **k != (ingr.to_owned(), aller.to_owned()))
                    .find(|(_, v)| *v == c)
                    .is_none()
            {
                v.push((ingr.to_owned(), aller.to_owned()));
            } else if count > 0 {
                if ingredients[i].len() == allergens[i].len() && ingredients[i].len() == 1 {
                    for ingr in &ingredients[i] {
                        for aller in &allergens[i] {
                            v.push((ingr.to_owned(), aller.to_owned()));
                        }
                    }
                }
            }

            for (ingr, aller) in &v {
                for i in 0..input.len() {
                    ingredients[i].remove(ingr);
                    allergens[i].remove(aller);
                }
            }
        }
    }

    let mut set = HashSet::new();
    for i in ingredients {
        for k in i {
            set.insert(k);
        }
    }

    let mut count = 0;
    for food in input {
        for i in &food.ingredients {
            if set.contains(i) {
                count += 1;
            }
        }
    }

    println!("part 1: {}", count);
}

fn part2(input: &[Food]) {
    let mut ingredients = input
        .iter()
        .map(|e| {
            e.ingredients
                .iter()
                .map(|e| e.to_owned())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();
    let mut allergens = input
        .iter()
        .map(|e| {
            e.allergens
                .iter()
                .map(|e| e.to_owned())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    let mut dangerous = vec![];

    while allergens.iter().map(|e| e.len()).sum::<usize>() > 0 {
        for i in 0..input.len() {
            if allergens[i].is_empty() {
                continue;
            }

            let mut v = vec![];
            let mut count_map = HashMap::new();
            let mut count = 0;
            for ingr in &ingredients[i] {
                for a in &allergens[i] {
                    count_map.insert((ingr.to_owned(), a.to_owned()), 0);
                }

                for j in 0..input.len() {
                    if j == i {
                        continue;
                    }

                    if ingredients[j].contains(ingr) {
                        count += 1;
                        for a in &allergens[i] {
                            if allergens[j].contains(a) {
                                *count_map
                                    .entry((ingr.to_owned(), a.to_owned()))
                                    .or_insert(0) += 1;
                            }
                        }
                    }
                }
            }

            let ((ingr, aller), c) = count_map
                .iter()
                .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
                .unwrap();
            if *c > 0
                && count_map
                    .iter()
                    .filter(|(k, _)| **k != (ingr.to_owned(), aller.to_owned()))
                    .find(|(_, v)| *v == c)
                    .is_none()
            {
                v.push((ingr.to_owned(), aller.to_owned()));
            } else if count > 0 {
                if ingredients[i].len() == allergens[i].len() && ingredients[i].len() == 1 {
                    for ingr in &ingredients[i] {
                        for aller in &allergens[i] {
                            v.push((ingr.to_owned(), aller.to_owned()));
                        }
                    }
                }
            }

            for (ingr, aller) in &v {
                dangerous.push((ingr.to_owned(), aller.to_owned()));
                for i in 0..input.len() {
                    ingredients[i].remove(ingr);
                    allergens[i].remove(aller);
                }
            }
        }
    }

    dangerous.sort_by(|(_, a), (_, b)| a.cmp(b));
    println!(
        "part 2: {}",
        dangerous
            .iter()
            .map(|(a, _)| a)
            .cloned()
            .collect::<Vec<_>>()
            .join(",")
    );
}
