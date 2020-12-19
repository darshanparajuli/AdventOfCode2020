use aoc_2020::read_input_map;
use std::collections::HashMap;

fn main() {
    let input = read_input_map(|e| e.chars().collect::<Vec<_>>());
    let input = Grid::new(input);
    part1(input.clone());
    part2(input.clone());
}

fn part1(mut input: Grid) {
    for _ in 0..6 {
        let next = input.get_next();

        // next.print();

        input = next;
    }

    let answer = input.grid.values().filter(|e| **e).count();
    println!("part 1: {}", answer);
}

fn part2(mut input: Grid) {
    for _ in 0..6 {
        let next = input.get_next_4d();

        // next.print();

        input = next;
    }

    let answer = input.grid.values().filter(|e| **e).count();
    println!("part 2: {}", answer);
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z, w: 0 }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: HashMap<Point, bool>,
}

impl Grid {
    fn new(input: Vec<Vec<char>>) -> Self {
        let mut grid = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                grid.insert(Point::new(x as i32, y as i32, 0), *c == '#');
            }
        }
        Self { grid }
    }

    fn active_neighbors(&self, point: &Point) -> u32 {
        let mut count = 0;

        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    let p = Point::new(point.x + x, point.y + y, point.z + z);
                    if p == *point {
                        continue;
                    }

                    if self.is_active(&p) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn active_neighbors_4d(&self, point: &Point) -> u32 {
        let mut count = 0;

        for w in -1..=1 {
            for z in -1..=1 {
                for y in -1..=1 {
                    for x in -1..=1 {
                        let mut p = Point::new(point.x + x, point.y + y, point.z + z);
                        p.w = point.w + w;

                        if p == *point {
                            continue;
                        }

                        if self.is_active(&p) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn get_next(&self) -> Self {
        let mut next = self.clone();

        let minz = self.grid.iter().map(|(k, _)| k.z).min().unwrap() - 1;
        let maxz = self.grid.iter().map(|(k, _)| k.z).max().unwrap() + 1;
        let miny = self.grid.iter().map(|(k, _)| k.y).min().unwrap() - 1;
        let maxy = self.grid.iter().map(|(k, _)| k.y).max().unwrap() + 1;
        let minx = self.grid.iter().map(|(k, _)| k.x).min().unwrap() - 1;
        let maxx = self.grid.iter().map(|(k, _)| k.x).max().unwrap() + 1;

        for z in minz..=maxz {
            for y in miny..=maxy {
                for x in minx..=maxx {
                    let p = Point::new(x, y, z);
                    let active_neighbors = self.active_neighbors(&p);
                    if self.is_active(&p) {
                        if active_neighbors != 2 && active_neighbors != 3 {
                            next.set_active(p, false);
                        }
                    } else {
                        if active_neighbors == 3 {
                            next.set_active(p, true);
                        }
                    }
                }
            }
        }

        next
    }

    fn get_next_4d(&self) -> Self {
        let mut next = self.clone();

        let minw = self.grid.iter().map(|(k, _)| k.w).min().unwrap() - 1;
        let maxw = self.grid.iter().map(|(k, _)| k.w).max().unwrap() + 1;
        let minz = self.grid.iter().map(|(k, _)| k.z).min().unwrap() - 1;
        let maxz = self.grid.iter().map(|(k, _)| k.z).max().unwrap() + 1;
        let miny = self.grid.iter().map(|(k, _)| k.y).min().unwrap() - 1;
        let maxy = self.grid.iter().map(|(k, _)| k.y).max().unwrap() + 1;
        let minx = self.grid.iter().map(|(k, _)| k.x).min().unwrap() - 1;
        let maxx = self.grid.iter().map(|(k, _)| k.x).max().unwrap() + 1;

        for w in minw..=maxw {
            for z in minz..=maxz {
                for y in miny..=maxy {
                    for x in minx..=maxx {
                        let mut p = Point::new(x, y, z);
                        p.w = w;
                        let active_neighbors = self.active_neighbors_4d(&p);
                        if self.is_active(&p) {
                            if active_neighbors != 2 && active_neighbors != 3 {
                                next.set_active(p, false);
                            }
                        } else {
                            if active_neighbors == 3 {
                                next.set_active(p, true);
                            }
                        }
                    }
                }
            }
        }

        next
    }

    #[allow(dead_code)]
    fn print(&self) {
        let minz = self.grid.iter().map(|(k, _)| k.z).min().unwrap();
        let maxz = self.grid.iter().map(|(k, _)| k.z).max().unwrap();
        let miny = self.grid.iter().map(|(k, _)| k.y).min().unwrap();
        let maxy = self.grid.iter().map(|(k, _)| k.y).max().unwrap();
        let minx = self.grid.iter().map(|(k, _)| k.x).min().unwrap();
        let maxx = self.grid.iter().map(|(k, _)| k.x).max().unwrap();

        for z in minz..=maxz {
            println!("z = {}", z);
            for y in miny..=maxy {
                for x in minx..=maxx {
                    let p = Point::new(x, y, z);

                    if self.is_active(&p) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }

    fn set_active(&mut self, p: Point, active: bool) {
        self.grid.insert(p, active);
    }

    fn is_active(&self, p: &Point) -> bool {
        *self.grid.get(&p).unwrap_or(&false)
    }
}
