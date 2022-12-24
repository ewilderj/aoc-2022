use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Op {
    Square,
    Multiply(u64),
    Add(u64),
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: usize,
    divisible_by: u64,
    throw_true: usize,
    throw_false: usize,
    op: Op,
}

impl Monkey {
    fn inspect(&self, old: u64) -> u64 {
        match self.op {
            Op::Square => old * old,
            Op::Add(b) => old + b,
            Op::Multiply(b) => old * b,
        }
    }
}

type MonkeyStash = HashMap<usize, RefCell<Vec<u64>>>;

fn main() {
    let mut ostash = MonkeyStash::new();
    let mut monkeys: Vec<Monkey> = Vec::new();

    for (i, m) in include_str!("../input.txt").split("\n\n").enumerate() {
        let mut l = m.lines().skip(1);
        let is: Vec<u64> = l
            .next()
            .unwrap()
            .get(18..)
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        // println!("{i} {:?}", &is);

        ostash.insert(i, RefCell::new(is));
        let ops: Vec<&str> = l.next().unwrap().get(23..).unwrap().split(" ").collect();

        let op: Op = match ops[0] {
            "*" => match ops[1].parse::<u64>() {
                Ok(n) => Op::Multiply(n),
                _ => Op::Square,
            },
            _ => Op::Add(ops[1].parse::<u64>().unwrap()),
        };

        let div_by = l.next().unwrap().get(21..).unwrap().parse::<u64>().unwrap();

        let (mt, mf) = (
            l.next()
                .unwrap()
                .get(29..)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            l.next()
                .unwrap()
                .get(30..)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );
        monkeys.push(Monkey {
            id: i,
            divisible_by: div_by,
            throw_true: mt,
            throw_false: mf,
            op: op,
        });
        // no monkey throws to itself
        assert!(i != mt as usize);
        assert!(i != mf as usize);
    }

    let ceiling: u64 = monkeys
        .iter()
        .map(|m| m.divisible_by)
        .reduce(|a, x| a * x)
        .unwrap();

    // parts 1 and 2 differ only in number of rounds
    // and function used to stop integer overflow
    for (part, maxr) in [(1, 20), (2, 10000)] {
        let stash = ostash.clone();
        let mut icount: HashMap<usize, u64> = HashMap::new();

        let limiter: Box<dyn Fn(u64) -> u64> = if part == 1 {
            Box::new(|z| z / 3)
        } else {
            Box::new(|z| z % ceiling)
        };

        for _ in 1..=maxr {
            for m in monkeys.iter() {
                // take a copy because Rust doesn't like mutating
                // what we're iterating over
                let items = stash[&m.id].borrow().clone();
                for &l in items.iter() {
                    let n = (limiter)(m.inspect(l));
                    *icount.entry(m.id).or_insert(0) += 1;
                    if n % m.divisible_by == 0 {
                        stash[&m.throw_true].borrow_mut().push(n);
                    } else {
                        stash[&m.throw_false].borrow_mut().push(n);
                    }
                }
                *stash[&m.id].borrow_mut() = Vec::new();
            }
        }

        println!("part{part}: {}", icount
            .values()
            .sorted()
            .rev()
            .take(2)
            .cloned()
            .reduce(|a, x| a * x)
            .unwrap());
    }
}
