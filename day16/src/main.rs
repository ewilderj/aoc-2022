use pathfinding::prelude::astar;
use regex::Regex;
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};

// This took a while! I tried a zillion wrong turns first, and eventually went
// looking for a bit of help after a hilarious failed genetic algorithm
// approach. Many thanks to Dazbo whose insights were useful.
//
// You can read their account for Python here: https://aoc.just2good.co.uk/2022/16
//
// I took a slightly different approach than Dazbo, who simulated the entire
// journey: instead I opted for pruning the 0-yielding valves away by
// computing the distance between yielding valves and only considering
// those transitions.
//
// What did I learn about Rust today?
// * How to compute a hash
// * BTreeSet, for a hashable set object

fn relief(
    t: i32,
    pos: &str,
    valves: &BTreeSet<&str>,
    elephant: bool,
    flow: &HashMap<&str, i32>,
    distance: &HashMap<(&str, &str), i32>,
    cache: &mut HashMap<u64, i32>,
) -> i32 {
    if t <= 0 {
        return 0;
    }

    // compute a hash for this call
    let mut h1 = DefaultHasher::new();
    t.hash(&mut h1);
    pos.hash(&mut h1);
    valves.hash(&mut h1);
    elephant.hash(&mut h1);
    let hk: u64 = h1.finish();

    if let Some(r) = cache.get(&hk) {
        return *r;
    } else {
        let released = t * flow[pos];
        let mut r = 0;

        // find the best sub-result, recursing
        for v in valves {
            let dt = distance.get(&(*v, pos)).unwrap() + 1;
            let rest: BTreeSet<&str> = valves.iter().filter(|c| *c != v).cloned().collect();
            r = cmp::max(r, relief(t - dt, v, &rest, elephant, flow, distance, cache));
        }

        // if we have an elephant, compare against the best result sending them now
        if elephant {
            r = cmp::max(r, relief(26, &"AA", valves, false, flow, distance, cache));
        }

        // when we get here, means it's time to open our valve!
        r += released;
        cache.insert(hk, r);
        return r;
    }
}

fn main() {
    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); .*valves? ([\w, ]+)").unwrap();
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flow: HashMap<&str, i32> = HashMap::new();
    let mut distance: HashMap<(&str, &str), i32> = HashMap::new();
    let mut cache: HashMap<u64, i32> = HashMap::new();
    for s in include_str!("../input.txt").lines() {
        assert!(re.is_match(s));
        let c = re.captures_iter(s).next().unwrap();
        let v = c.get(1).unwrap().as_str();
        flow.insert(v, c.get(2).unwrap().as_str().parse::<i32>().unwrap());
        neighbors.insert(
            v,
            c.get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .collect::<Vec<&str>>(),
        );
    }

    // use A* to compute shortest distance from one place to another
    // N is small enough that I don't care to remove dupes
    for v in neighbors.keys() {
        for w in neighbors.keys() {
            if v == w {
                distance.insert((*v, *w), 0);
            } else {
                let r = astar(
                    v,
                    |p| neighbors.get(p).unwrap().iter().map(|i| (*i, 1)),
                    |_| 1,
                    |p| p == w,
                );
                distance.insert((*v, *w), r.unwrap().1);
            }
        }
    }

    // we need to optimize a journey between yielding-valves so as to maximize
    // the resulting score.
    let valves: BTreeSet<&str> = flow
        .iter()
        .filter(|(_, &f)| f > 0)
        .map(|(k, _)| *k)
        .collect();

    println!(
        "part1: {}",
        relief(30, &"AA", &valves, false, &flow, &distance, &mut cache)
    );

    println!(
        "part2: {}",
        relief(26, &"AA", &valves, true, &flow, &distance, &mut cache)
    );
}
