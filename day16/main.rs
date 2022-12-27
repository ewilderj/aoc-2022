use pathfinding::prelude::astar;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use permute::permutations_of;

fn main() {
    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); .*valves? ([\w, ]+)").unwrap();
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flow: HashMap<&str, i32> = HashMap::new();
    let mut distance: HashMap<(&str, &str), i32> = HashMap::new();
    let mut opened: HashSet<&str> = HashSet::new();
    for s in include_str!("../test.txt").lines() {
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
    // println!("{:?}\n{:?}", flow, neighbors);

    // for funsies, compute shortest distance from one place to another
    for v in neighbors.keys() {
        for w in neighbors.keys() {
            if v == w {
                distance.insert((*v, *w), 0);
            } else {
                let r = astar(
                    v,
                    |p| neighbors.get(p).unwrap().iter().map(|i| (*i, 1)),
                    |p| 1,
                    |p| p == w,
                );
                distance.insert((*v, *w), r.unwrap().1);
            }
        }
    }

    println!("{:?}", distance);

    // we need to optimize a journey between yielding-valves so as to maximize
    // the resulting score. smells super NP-complete to me!
    let opts = flow.iter().filter(|(_, &f)| f > 0).map(|(k, _)| *k).collect::<Vec<&str>>();

    println!("valves to perm: {:?}", opts);

    fn score_solution(soln: &Vec<&str>, flow: &HashMap<&str, i32>, distance: &HashMap<(&str, &str), i32>) -> i32 {
        let mut score = 0;
        let mut pos = &"AA";
        let mut m = 0;

        for v in soln.iter() {
            let f = flow[v];
            let d = distance.get(&(*v, pos)).unwrap();
            let s = f * (29 - m - d);
            println!("At t={m}, {} --{d}--> {} yields {}", pos, v, s);
            pos = v;
            score += s;
            m += d + 1;
            if m > 30 {
                score = 0;
                break;
            }
            return score;
        }
    }

    // Here's a brute force solution that only works to verify the test data: it searches
    // all permutations
    let mut best = 0;
    for p in permutations_of(&opts) {
        let c = p.collect::Vec<&str>();
        let score = score_solution(&c, &flow, &distance);
        if score > best {
            best = score;
        }
    }
    println!("best: {}", best);

    // yeah, this ain't working... simply picking the best option
    // each time causes future pain. so we somehow need to do a backtracking
    // search to find the best path.
    //
    // TODO: tomorrow, permute all the rooms with valves > 0 and compute
    // the cost of those sequences, choose the best.






    //    let mut m = 0;
//     let mut v = &"AA";
//     while m < 30 {
//         println!("minute {}", m + 1);
//         let mut k = valves
//             .iter()
//             .filter(|w| w != &v)
//             .map(|w| (flow[w] * (29 - m - distance.get(&(v, w)).unwrap()), w))
//             .collect::<Vec<_>>();
//         k.sort();
//         println!("{:?}", k);
//         m += 1;
//     }
 }
