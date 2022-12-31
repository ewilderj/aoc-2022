use regex::Regex;
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// This is a pretty slow search, though it runs in seconds.
// Other people have gotten millisecond execution with smarter
// strategies for pruning the search space.
//
// For instance, I do nothing with the knowledge of the time
// remaining, but rely on recursion to do all the calculation...

#[derive(Debug, PartialEq, Clone)]
struct Blueprint {
    id: u32,
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost: (u32, u32),
    geode_cost: (u32, u32),
}

type Cache = HashMap<u64, u32>;

fn search(
    t: u32,
    c: &mut Cache,
    b: &Blueprint,
    robots: (u32, u32, u32, u32),
    resources: (u32, u32, u32, u32),
) -> u32 {
    let (ore, clay, obsidian, geode) = resources;

    if t == 0 {
        return geode;
    }

    let mut h1 = DefaultHasher::new();
    t.hash(&mut h1);
    b.id.hash(&mut h1);
    robots.hash(&mut h1);
    resources.hash(&mut h1);
    let hk: u64 = h1.finish();

    if let Some(r) = c.get(&hk) {
        return *r;
    }

    let (r_ore, r_clay, r_obsidian, r_geode) = robots;

    let nr = (
        ore + r_ore,
        clay + r_clay,
        obsidian + r_obsidian,
        geode + r_geode,
    );

    let mut r_max = 0;

    // now let's explore decision options

    // buy new geode bot if we can!
    // it works as a heuristic to not buy other options for this puzzle,
    // though it's not necessarily a correct assumption for larger N
    if ore >= b.geode_cost.0 && obsidian >= b.geode_cost.1 {
        r_max = cmp::max(
            r_max,
            search(
                t - 1,
                c,
                b,
                (r_ore, r_clay, r_obsidian, r_geode + 1),
                (nr.0 - b.geode_cost.0, nr.1, nr.2 - b.geode_cost.1, nr.3),
            ),
        );
    } else {

        // consider the supply chain purposes
        // trimming with some guesses: we don't need to build new
        // bots once we're 2 hops down the chain, and don't need
        // to exceed the per-turn supply requirements

        if ore >= b.ore_cost && r_ore < 4 && r_obsidian == 0 {
            r_max = cmp::max(
                r_max,
                search(
                    t - 1,
                    c,
                    b,
                    (r_ore + 1, r_clay, r_obsidian, r_geode),
                    (nr.0 - b.ore_cost, nr.1, nr.2, nr.3),
                ),
            );
        }

        if ore >= b.clay_cost && r_clay < 20 && r_geode == 0 {
            r_max = cmp::max(
                r_max,
                search(
                    t - 1,
                    c,
                    b,
                    (r_ore, r_clay + 1, r_obsidian, r_geode),
                    (nr.0 - b.clay_cost, nr.1, nr.2, nr.3),
                ),
            );
        }

        if ore >= b.obsidian_cost.0 && clay >= b.obsidian_cost.1 && r_obsidian < 20 {
            r_max = cmp::max(
                r_max,
                search(
                    t - 1,
                    c,
                    b,
                    (r_ore, r_clay, r_obsidian + 1, r_geode),
                    (
                        nr.0 - b.obsidian_cost.0,
                        nr.1 - b.obsidian_cost.1,
                        nr.2,
                        nr.3,
                    ),
                ),
            );
        }

        // finally compare against just running the clock, but not worth the
        // comparison if we already have enough clay (obsidian is considered
        // in the other arm of this outer if-else)
        if clay < 20 {
            r_max = cmp::max(r_max, search(t - 1, c, b, robots, nr));
        }

    }
    c.insert(hk, r_max);

    r_max
}

fn main() {
    let mut c = Cache::new();

    let re = Regex::new(r"(\d+):\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)").unwrap();
    let s = include_str!("../input.txt")
        .lines()
        .map(|s| {
            assert!(re.is_match(s));
            let c = re.captures_iter(s).next().unwrap();
            Blueprint {
                id: c.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                ore_cost: c.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                clay_cost: c.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                obsidian_cost: (
                    c.get(4).unwrap().as_str().parse::<u32>().unwrap(),
                    c.get(5).unwrap().as_str().parse::<u32>().unwrap(),
                ),
                geode_cost: (
                    c.get(6).unwrap().as_str().parse::<u32>().unwrap(),
                    c.get(7).unwrap().as_str().parse::<u32>().unwrap(),
                ),
            }
        })
        .collect::<Vec<Blueprint>>();

    let mut q = 0;
    for b in s.iter() {
        let r = search(24, &mut c, b, (1, 0, 0, 0), (0, 0, 0, 0));
        // println!("b{} {:?}", b.id, r);
        q += b.id * r;
    }
    println!("part1: {}", q);

    q = 1;
    for b in s.iter().take(3) {
        let r = search(32, &mut c, b, (1, 0, 0, 0), (0, 0, 0, 0));
        // println!("b{} {:?}", b.id, r);
        q *= r;
    }
    println!("part2: {}", q);
}
