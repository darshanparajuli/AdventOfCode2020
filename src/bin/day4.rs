use aoc_2020::read_input;
use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<String> = read_input().iter().map(|e| e.to_owned()).collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &[String]) {
    let mut passports = vec![];

    let mut index = 0;
    while index < input.len() {
        let mut keys = HashSet::new();

        while index < input.len() {
            if input[index].is_empty() {
                break;
            }

            let s = &input[index];
            s.split(" ").for_each(|e| {
                let key = e.split(":").next().unwrap();
                keys.insert(key.to_owned());
            });

            index += 1;
        }

        passports.push(keys);
        index += 1;
    }

    let count = passports
        .iter()
        .filter(|e| e.len() == 8 || (e.len() == 7 && !e.contains("cid")))
        .count();
    println!("part 1: {}", count);
}

fn part2(input: &[String]) {
    let mut passports = vec![];

    let mut index = 0;
    while index < input.len() {
        let mut map = HashMap::new();

        while index < input.len() {
            if input[index].is_empty() {
                index += 1;
                break;
            }

            let s = &input[index];
            s.split(" ").for_each(|e| {
                let key_value = e.split(":").collect::<Vec<_>>();
                map.insert(key_value[0], key_value[1]);
            });

            index += 1;
        }

        if !map.is_empty() {
            passports.push(map);
        }
    }

    let ecl_set = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .collect::<HashSet<_>>();
    let count = passports
        .iter()
        .filter(|e| e.len() == 8 || (e.len() == 7 && !e.contains_key("cid")))
        .filter(|e| {
            for (k, v) in e.iter() {
                match *k {
                    "byr" => {
                        if let Ok(value) = v.parse::<u32>() {
                            if v.len() != 4 {
                                return false;
                            }
                            if value < 1920 || value > 2002 {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    "iyr" => {
                        if let Ok(value) = v.parse::<u32>() {
                            if v.len() != 4 {
                                return false;
                            }
                            if value < 2010 || value > 2020 {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    "eyr" => {
                        if let Ok(value) = v.parse::<u32>() {
                            if v.len() != 4 {
                                return false;
                            }
                            if value < 2020 || value > 2030 {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    "hgt" => {
                        if v.contains("cm") {
                            let i = v.find("cm").unwrap();
                            if i == 0 {
                                return false;
                            }
                            if let Ok(h) = v[0..i].parse::<u32>() {
                                if h < 150 || h > 193 {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        } else if v.contains("in") {
                            let i = v.find("in").unwrap();
                            if i == 0 {
                                return false;
                            }
                            if let Ok(h) = v[0..i].parse::<u32>() {
                                if h < 59 || h > 76 {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    "hcl" => {
                        if !v.starts_with("#") {
                            return false;
                        }

                        let count = v
                            .chars()
                            .skip(1)
                            .filter(|e| match e {
                                '0'..='9' | 'a'..='f' => true,
                                _ => false,
                            })
                            .count();
                        if count != 6 {
                            return false;
                        }
                    }
                    "ecl" => {
                        if !ecl_set.contains(v) {
                            return false;
                        }
                    }
                    "pid" => {
                        let count = v
                            .chars()
                            .filter(|e| match e {
                                '0'..='9' => true,
                                _ => false,
                            })
                            .count();
                        if count != 9 {
                            return false;
                        }
                    }
                    "cid" => {}
                    _ => unreachable!(),
                }
            }
            true
        })
        .count();
    println!("part 2: {}", count);
}
