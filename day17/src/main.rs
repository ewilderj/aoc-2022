use std::collections::HashMap;

type Rock = [(i64, i64)];
type RockMap = HashMap<(i64, i64), char>;

// Well, this was a clumsy one. Everything went well for part1, was quite proud
// of a neat little simulation, and a nice debugging visualization routine.
//
// then I managed to find an approximatation for the answer
// which for test data was h = 3n/2 + n/70 + 7
// but it was off by a few and not exact
//
// so, then it became more about brute force cycle finding, and the
// mathing is rather messy but it works, and it doesn't seem like a
// lot of fun to tidy it up.
//
// also this was a prime problem to use bit math with, but i didn't
// you can see i decided to convert to bits for the cycle detection
// just to make life simpler, and i suppose i could backport that
// to part1, but life is short and this stopped being fun for me :)

// rules for rocks (h, w) is the first pair, then coords
const ROCK1: [(i64, i64); 5] = [(4, 1), (0, 0), (1, 0), (2, 0), (3, 0)];
const ROCK2: [(i64, i64); 6] = [(3, 3), (1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
const ROCK3: [(i64, i64); 6] = [(3, 3), (0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const ROCK4: [(i64, i64); 5] = [(1, 4), (0, 0), (0, 1), (0, 2), (0, 3)];
const ROCK5: [(i64, i64); 5] = [(2, 2), (0, 0), (1, 0), (0, 1), (1, 1)];
const ROCKS: [&Rock; 5] = [&ROCK1, &ROCK2, &ROCK3, &ROCK4, &ROCK5];

fn collides(grid: &RockMap, rock: &Rock, x: i64, y: i64) -> bool {
    rock[1..]
        .iter()
        .map(|(rx, ry)| (rx + x, ry + y))
        .map(|(p, q)| q < 0 || grid.contains_key(&(p, q)))
        .any(|b| b)
}

fn output(grid: &RockMap, max_y: i64) {
    for y in 0..=max_y {
        print!("{}\t|", max_y - y);
        for x in 0..=6 {
            if grid.contains_key(&(x, max_y - y)) {
                print!("{}", *grid.get(&(x, max_y - y)).unwrap());
            } else {
                print!(".")
            }
        }
        println!("|");
    }
    println!("\t+-------+");
}

fn iterate(lim: usize, grid: &mut RockMap, heights: &mut HashMap<usize, usize>, s: &str) -> i64 {
    let mut jets = s.chars().cycle();
    let mut max_y: i64 = 0;
    let mut rock: usize = 0;

    while rock < lim {
        let mut rx: i64 = 2;
        let mut ry: i64 = 3 + max_y;
        let r: &Rock = ROCKS[rock % 5];
        let lhs = 7 - r[0].0;

        loop {
            let crx = match jets.next() {
                Some('>') => {
                    if rx < lhs {
                        rx + 1
                    } else {
                        rx
                    }
                }
                _ => {
                    if rx > 0 {
                        rx - 1
                    } else {
                        rx
                    }
                }
            };
            // ensure wind doesn't blow us into rock
            if !collides(&grid, r, crx, ry) {
                rx = crx;
            }

            if collides(&grid, r, rx, ry - 1) {
                for (x, y) in r.iter().skip(1) {
                    grid.insert(
                        (x + rx, y + ry),
                        std::char::from_digit(rock as u32 % 5 + 1, 10).unwrap(),
                    );
                }

                max_y = std::cmp::max(max_y, ry + r[0].1);
                heights.insert(max_y as usize, rock + 1); // +1 cos of zero-based math
                break;
            } else {
                ry -= 1;
            }
        }
        rock += 1;
    }
    max_y
}

fn main() {
    let s = include_str!("../input.txt").trim();

    let mut grid: RockMap = HashMap::new();
    let mut h2r: HashMap<usize, usize> = HashMap::new();
    let mut coded: Vec<u8> = vec![];

    println!("part1: {}", iterate(2022, &mut grid, &mut h2r, &s));

    // start again for part 2
    grid = HashMap::new();
    h2r = HashMap::new();

    // do enough iterations that we might find a pattern
    let maxy = iterate(8192, &mut grid, &mut h2r, &s);

    // set up another hash to invert h2r
    let mut r2h: HashMap<usize, usize> = HashMap::new();

    // gonna convert my hashmap to a byte vector
    // for searching: tbh could have used that all the
    // way through but not keen to do a part1 rewrite
    for y in 0..=maxy {
        // invert rocks/heights while we're here
        if let Some(r) = h2r.get(&(y as usize)) {
            r2h.insert(*r, y as usize);
        }
        let b: u8 = (0..7)
            .map(|n| {
                if grid.contains_key(&(n, y)) {
                    1 << (6 - n)
                } else {
                    0
                }
            })
            .sum();
        coded.push(b);
    }

    let mut r0: usize = 0;
    let mut dr = 0;
    let mut dh: usize = 0;

    // look for a repeating pattern of rocks
    'outer: for prefix in 0..4096 {
        for period in 1..4096 {
            let found = (0..200) // not exhaustive, but good enough
                .map(|i| coded[i + prefix] == coded[i + prefix + period])
                .all(|x| x);
            if found {
                // rocks before the cycle starts
                r0 = h2r[&prefix] as usize;
                // how many rocks it takes to build delta-height
                dr = (h2r[&(period + prefix)] - h2r[&prefix]) as usize;
                // delta-height
                dh = period;
                break 'outer;
            }
        }
    }

    let target = 1000000000000usize;
    let t1 = target - r0; // remove prefix number of rocks
    let tm = t1 / dr; // how many cycles
    let tr = t1 % dr; // how many rocks left over
    let topup = r2h[&(tr + r0)]; // how much extra height bc of `tr` and `r0` (leftover and prefix #)

    println!("part2: {}", topup + tm * dh);
}
