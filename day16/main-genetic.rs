use itertools::max;
use pathfinding::prelude::astar;
use permute::permutations_of;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::collections::HashMap;

fn score_solution(
    soln: &Vec<&str>,
    flow: &HashMap<&str, i32>,
    distance: &HashMap<(&str, &str), i32>,
) -> i32 {
    let mut score = 0;
    let mut pos = &"AA";
    let mut m = 0;

    for v in soln.iter() {
        let f = flow[v];
        let d = distance.get(&(*v, pos)).unwrap();
        let s = f * (29 - m - d);
        // println!("At t={m}, {} --{d}--> {} yields {}", pos, v, s);
        pos = v;
        m += d + 1;
        if m >= 30 {
            break;
        }
        score += s;
    }
    return score;
}

fn main() {
    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); .*valves? ([\w, ]+)").unwrap();
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flow: HashMap<&str, i32> = HashMap::new();
    let mut distance: HashMap<(&str, &str), i32> = HashMap::new();

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
    // println!("{:?}\n{:?}", flow, neighbors);

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
    // the resulting score. smells super NP-complete to me -- as there are N!
    // possible orderings
    let valves = flow
        .iter()
        .filter(|(_, &f)| f > 0)
        .map(|(k, _)| *k)
        .collect::<Vec<&str>>();

    let n = valves.len();
    println!("{n} valves to perm: {:?}", valves);

    const POP: usize = 20;

    fn crossover<'a>(a: &Vec<&'a str>, b: &Vec<&'a str>) -> Vec<&'a str> {
        // id a substring to splice in
        let n = 5; // thread_rng().gen_range(2..5);
        let i = thread_rng().gen_range(0..a.len() - n);
        let x: Vec<&str> = a.iter().skip(i).take(n).cloned().collect();
        let r: Vec<&str> = b
            .iter()
            .take(i)
            .filter(|c| !x.contains(c))
            .chain(x.iter())
            .chain(b.iter().skip(i).filter(|c| !x.contains(c)))
            .cloned()
            .collect();
        // println!("crossover {:?} {:?} -> {:?}", a, b, r);
        r
    }

    fn random_swap<'a>(a: &Vec<&'a str>) -> Vec<&'a str> {
        let mut r = a.clone();
        let p = thread_rng().gen_range(0..a.len() - 1);
        r[p] = a[p + 1];
        r[p + 1] = a[p];
        // println!("{:?} => {:?}", a, r);
        r
    }

    let mut best: i32 = 0;
    let mut population = Vec::new();

    for sim in 1..20 {
        population = Vec::new();
        for _ in 0..POP {
            let mut p = valves.clone();
            p.shuffle(&mut thread_rng());
            population.push(p);
        }

        let mut generations = 0;
        let mut fitness: HashMap<usize, i32> = HashMap::new();
        let mut max_score = 0;
        let mut fittest: usize = 0;
        let mut second_fittest: usize = 0;
        let mut weakest: usize = 0;
        let mut solns: Vec<usize> = (0..POP).collect();
        loop {
            for i in 0..POP {
                fitness.insert(i, score_solution(&population[i], &flow, &distance));
            }
            solns.sort_by(|a, b| fitness[b].cmp(&fitness[a]));
            fittest = solns[0];
            second_fittest = solns[1];

            if generations > 100000 {
                println!(
                    "fittest {}={:?} {:?}",
                    fittest, fitness[&fittest], population[fittest]
                );
                break;
            }

            // update weakest with crossover, score again
            population[solns[POP - 1]] =
                crossover(&population[fittest], &population[second_fittest]);

            if thread_rng().gen_range(0..7) < 4 {
                // mutate: swap pairs at a random point in
                // fittest and second fittest and override the weakest populations
                population[solns[POP - 2]] = random_swap(&population[fittest]);
                population[solns[POP - 3]] = random_swap(&population[second_fittest]);
            }

            generations += 1;
        }
        if fitness[&fittest] > best {
            best = fitness[&fittest];
        }
    }

    println!("best = {best}");

    // Here's a brute force solution that only works to verify the test data: it searches
    // all permutations
    let mut best: i32 = 0;

    assert!(n < 7); // just break if we need a power plant to process
    for p in permutations_of(&valves) {
        let c = p.cloned().collect::<Vec<&str>>();
        let score: i32 = score_solution(&c, &flow, &distance);
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
