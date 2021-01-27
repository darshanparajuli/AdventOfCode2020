use aoc_2020::*;

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

#[derive(Debug, Copy, Clone)]
struct Cup {
    value: u64,
    next: RawPtr<Cup>,
}

impl Cup {
    fn new(value: u64) -> Self {
        Self {
            value,
            next: RawPtr::null(),
        }
    }
}

struct CrabCups {
    head: RawPtr<Cup>,
    picked: RawPtr<Cup>,
    cup_map: Vec<RawPtr<Cup>>,
    max_num: u64,
}

impl CrabCups {
    fn new(input: &[u64]) -> Self {
        let mut head: RawPtr<Cup> = RawPtr::null();
        let mut last: RawPtr<Cup> = RawPtr::null();
        let mut cup_map = vec![RawPtr::null(); input.len()];

        for n in input {
            let cup = RawPtr::from_boxed(Box::new(Cup::new(*n)));
            if head.is_null() {
                head = cup;
                last = cup;
            } else {
                last.as_mut().next = cup;
                last = cup;
            }

            cup_map[(*n - 1) as usize] = cup;
        }

        last.as_mut().next = head;
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
        let dest_cup = self.get_dest_cup(picked_three);

        let (a, _, c) = picked_three;
        let next = dest_cup.as_ref().next;
        dest_cup.as_mut().next = a;
        c.as_mut().next = next;

        self.picked = self.picked.as_ref().next;
    }

    fn get_dest_cup(
        &mut self,
        picked_three: (RawPtr<Cup>, RawPtr<Cup>, RawPtr<Cup>),
    ) -> RawPtr<Cup> {
        let (a, b, c) = picked_three;

        let a_v = a.as_ref().value;
        let b_v = b.as_ref().value;
        let c_v = c.as_ref().value;

        let mut target = self.picked.as_ref().value - 1;
        while target > 0 && (target == a_v || target == b_v || target == c_v) {
            target -= 1;
        }

        if target == 0 {
            let mut max = self.max_num;
            while max == a_v || max == b_v || max == c_v {
                max -= 1;
            }
            self.cup_map[(max as usize) - 1]
        } else {
            self.cup_map[(target as usize) - 1]
        }
    }

    fn take_next_three(&mut self) -> (RawPtr<Cup>, RawPtr<Cup>, RawPtr<Cup>) {
        let a = self.picked.as_ref().next;
        let b = a.as_ref().next;
        let c = b.as_ref().next;

        self.picked.as_mut().next = c.as_ref().next;
        c.as_mut().next = RawPtr::null();

        (a, b, c)
    }

    fn get_values_after_one(&self) -> Vec<u64> {
        let mut result = vec![];
        let start = self.cup_map[0];
        let mut next = start.as_ref().next;

        while next.ptr() != start.ptr() {
            let value = next.as_ref().value;
            result.push(value);

            next = next.as_ref().next;
        }

        result
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut node = self.head;
        let mut i = 0;
        let picked_value = self.picked.as_ref().value;
        while i < self.cup_map.len() {
            let v = node.as_ref().value;
            if v == picked_value {
                print!("({}) ", v);
            } else {
                print!(" {}  ", v);
            }
            node = node.as_ref().next;
            i += 1;
        }
        println!();
    }
}

impl Drop for CrabCups {
    fn drop(&mut self) {
        let count = self.cup_map.len();
        self.cup_map.clear();
        self.picked = RawPtr::null();
        self.max_num = 0;

        let mut n = self.head;
        for _ in 0..count {
            let k = n;
            n = n.as_ref().next;
            drop(k.into_boxed());
        }

        self.head = RawPtr::null();
    }
}
