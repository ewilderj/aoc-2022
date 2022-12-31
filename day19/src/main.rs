use regex::Regex;
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

    if t <= 0 {
        // if geode >1 {
        //     println!("MAX: Minute {t}\nResources: {:?}\nRobots: {:?}", resources, robots);
        // }
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

    // println!("Minute {t}\nResources: {:?}\nRobots: {:?}", resources, robots);

    let (r_ore, r_clay, r_obsidian, r_geode) = robots;

    let nr = (
        ore + r_ore,
        clay + r_clay,
        obsidian + r_obsidian,
        geode + r_geode,
    );

    let mut r_max = 0;
    let mut note = String::new();

    // now let's just perm the available choices

    // #1 - use money to buy a new orebot, but also not bother if ore is growing
    if ore >= b.ore_cost  {
        let r = search(
            t - 1,
            c,
            b,
            (r_ore + 1, r_clay, r_obsidian, r_geode),
            (nr.0 - b.ore_cost, nr.1, nr.2, nr.3),
        );
        if r > r_max {
            note = format!("Time {t}: bought ore robot");
            r_max = r;
        }
    }

    // #2 - use money to buy a new claybot
    if ore >= b.clay_cost {
        let r = search(
            t - 1,
            c,
            b,
            (r_ore, r_clay + 1, r_obsidian, r_geode),
            (nr.0 - b.clay_cost, nr.1, nr.2, nr.3),
        );

        if r > r_max {
            note = format!("Time {t}: bought clay robot");
            r_max = r;
        }
    }

    // #3 - buy new obsidian bot
    if ore >= b.obsidian_cost.0 && clay >= b.obsidian_cost.1 {
        let r = search(
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
        );
        if r > r_max {
            note = format!("Time {t}: bought obsidian robot");
            r_max = r;
        }
    }

    // #4 - buy new geode bot
    if ore >= b.geode_cost.0 && obsidian >= b.geode_cost.1 {
        let r = search(
            t - 1,
            c,
            b,
            (r_ore, r_clay, r_obsidian, r_geode + 1),
            (nr.0 - b.geode_cost.0, nr.1, nr.2 - b.geode_cost.1, nr.3),
        );
        if r > r_max {
            note = format!("Time {t}: bought geode robot");
            r_max = r;
        }
    }

    if ore < 30 {
        r_max = std::cmp::max(r_max, search(t - 1, c, b, robots, nr));
    }
    c.insert(hk, r_max);

    return r_max;
}

fn main() {
    let mut c = Cache::new();

    //     println!("{:?}", search(0, &mut c, &b1, (1, 0, 0, 0), (0, 0, 0, 0)));

    let re =
        Regex::new(r"Blueprint (\d+):\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)").unwrap();
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
        let r = search(24, &mut c, &b, (1, 0, 0, 0), (0, 0, 0, 0));
        println!("b{} {:?}", b.id, r);
        q = q + b.id * r;
    }
    println!("part1: {}", q);
    // prt2: we have 32 minutes now
    q = 1;
    for b in s.iter().take(3) {
        let r = search(32, &mut c, &b, (1, 0, 0, 0), (0, 0, 0, 0));
        println!("b{} {:?}", b.id, r);
        q = q * r;
    }
    println!("part2: {}", q);

}


// 34632 -- too low
