use aoc_2020::*;
use std::ptr;

fn main() {
    let input = read_input()
        .first()
        .unwrap()
        .chars()
        .map(|e| e.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    part1(input.clone());
    part2(input);
}

fn part1(input: Vec<u64>) {
    let mut crab_cups = CrabCups::new(&input);
    for _ in 0..100 {
        crab_cups.play_round();
    }
    println!(
        "part 1: {}",
        crab_cups
            .get_values_after_one()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}

fn part2(input: Vec<u64>) {
    let mut input = input.to_vec();
    let max = input.iter().max().unwrap();
    for i in max + 1..=1_000_000 {
        input.push(i);
    }

    let mut crab_cups = CrabCups::new(&input);
    for _ in 0..10_000_000 {
        crab_cups.play_round();
    }

    let values_after_one = crab_cups.get_values_after_one();
    let a = values_after_one[0];
    let b = values_after_one[1];
    println!("part 2: {}", a * b);
}

#[derive(Debug, Clone)]
struct Cup {
    value: u64,
    next: *mut Cup,
}

impl Cup {
    fn new(value: u64) -> Self {
        Self {
            value,
            next: ptr::null_mut(),
        }
    }
}

struct CrabCups {
    head: *mut Cup,
    picked: *mut Cup,
    cup_map: Vec<*const Cup>,
    max_num: u64,
}

impl CrabCups {
    fn new(input: &[u64]) -> Self {
        let mut head: *mut Cup = ptr::null_mut();
        let mut last: *mut Cup = ptr::null_mut();
        let mut cup_map = vec![ptr::null(); input.len()];

        for n in input {
            let cup = Box::new(Cup::new(*n));
            let cup = Box::into_raw(cup);
            if head.is_null() {
                head = cup;
                last = cup;
            } else {
                {
                    let last = unsafe { last.as_mut().unwrap() };
                    last.next = cup;
                }
                last = cup;
            }

            cup_map[(*n - 1) as usize] = cup;
        }

        {
            let last = unsafe { last.as_mut().unwrap() };
            last.next = head;
        }

        let max_num = *input.iter().max().unwrap();

        Self {
            head,
            picked: head,
            cup_map,
            max_num,
        }
    }

    fn play_round(&mut self) {
        let picked_three = self.take_next_three();
        let dest_cup = self.get_dest_cup(&picked_three);

        let (a, _, c) = picked_three;
        let next = unsafe { dest_cup.as_ref().unwrap() }.next;
        unsafe { dest_cup.as_mut().unwrap() }.next = a;
        unsafe { c.as_mut().unwrap() }.next = next;

        self.pick_next();
    }

    fn pick_next(&mut self) {
        let picked = unsafe { self.picked.as_ref().unwrap() };
        self.picked = picked.next;
    }

    fn get_dest_cup(&mut self, picked_three: &(*mut Cup, *mut Cup, *mut Cup)) -> *mut Cup {
        let (a, b, c) = picked_three;

        let a_v = unsafe { a.as_ref().unwrap() }.value;
        let b_v = unsafe { b.as_ref().unwrap() }.value;
        let c_v = unsafe { c.as_ref().unwrap() }.value;

        let mut target = unsafe { self.picked.as_ref().unwrap() }.value - 1;
        while target > 0 && (target == a_v || target == b_v || target == c_v) {
            target -= 1;
        }

        if target == 0 {
            let mut max = self.max_num;
            while max == a_v || max == b_v || max == c_v {
                max -= 1;
            }
            self.cup_map[(max as usize) - 1] as *mut _
        } else {
            self.cup_map[(target as usize) - 1] as *mut _
        }
    }

    fn take_next_three(&mut self) -> (*mut Cup, *mut Cup, *mut Cup) {
        let a = unsafe { self.picked.as_ref().unwrap() }.next;
        let b = unsafe { a.as_ref().unwrap() }.next;
        let c = unsafe { b.as_ref().unwrap() }.next;

        unsafe { self.picked.as_mut().unwrap() }.next = unsafe { c.as_ref().unwrap() }.next;
        unsafe { c.as_mut().unwrap() }.next = ptr::null_mut();

        (a, b, c)
    }

    fn get_values_after_one(&self) -> Vec<u64> {
        let mut result = vec![];
        let start = self.cup_map[0];
        let mut next = unsafe { start.as_ref().unwrap() }.next as *const _;

        while next != start {
            let value = unsafe { next.as_ref().unwrap() }.value;
            result.push(value);

            next = unsafe { next.as_ref().unwrap() }.next;
        }

        result
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut node = self.head;
        let mut i = 0;
        let picked_value = unsafe { self.picked.as_ref().unwrap() }.value;
        while i < self.cup_map.len() {
            let v = unsafe { node.as_ref().unwrap() }.value;
            if v == picked_value {
                print!("({}) ", v);
            } else {
                print!(" {}  ", v);
            }
            node = unsafe { node.as_ref().unwrap() }.next;
            i += 1;
        }
        println!();
    }
}

impl Drop for CrabCups {
    fn drop(&mut self) {
        let count = self.cup_map.len();
        self.cup_map.clear();
        self.picked = ptr::null_mut();
        self.max_num = 0;

        let mut n = self.head;
        for _ in 0..count {
            let k = n;
            n = unsafe { n.as_ref().unwrap() }.next;
            drop(unsafe { Box::from_raw(k) });
        }

        self.head = ptr::null_mut();
    }
}
