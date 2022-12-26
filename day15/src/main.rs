use regex::Regex;
use std::collections::{HashMap, HashSet};

// Well, this was a fun one.
// As I did some stupid (but correct) solutions I started
// to understand the problem better and eventually came
// to the current solution, computing over ranges.
//
// Initial brute force approaches were based on plotting
// and computing the state at every coordinate. That was
// viable for part1 but not for part2.

type SensorMap = HashMap<(i32, i32), i32>;

fn md(x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    (x0.abs_diff(x1) + y0.abs_diff(y1)) as i32
}

// compute ranges at a particular y coord
fn coverage(sensors: &SensorMap, y: i32) -> Vec<(i32, i32)> {
    let mut xs: Vec<(i32, i32)> = Vec::new();
    for ((sx, sy), r) in sensors.iter() {
        let dy = y.abs_diff(*sy) as i32;
        if dy <= *r {
            // record x range of this sensor
            let (t, f) = (*sx + *r - dy, *sx - *r + dy);
            xs.push((f, t));
        }
    }
    xs.sort();
    xs
}

// given a sorted array of ranges, resolve overlaps
fn canonicalize(xs: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut r: Vec<(i32, i32)> = vec![xs[0].clone()];

    for i in 1..xs.len() {
        let (a, b) = r[r.len() - 1];
        let (f, t) = xs[i];
        if f > b + 1 {
            // discontinuous so start a new range
            r.push((f, t));
        } else {
            // update current range to extend it
            if t > b {
                r.pop();
                r.push((a, t));
            }
        }
    }
    r
}

fn main() {
    let mut sensors: SensorMap = HashMap::new();
    let re = Regex::new(r"=(\-?\d+)\D+=(\-?\d+)\D+=(\-?\d+)\D+=(\-?\d+)").unwrap();
    let beacons: HashSet<(i32, i32)> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let c = re.captures_iter(s).next().unwrap();
            let (sx, sy, bx, by) = (
                c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            );
            // NB: side effect: store sensor and the distance from its
            // nearest beacon.
            sensors.insert((sx, sy), md(sx, sy, bx, by));
            (bx, by)
        })
        .collect();

    const TROW: i32 = 2000000; // 2000000

    let taken: usize = beacons.iter().filter(|(_, y)| *y == TROW).count()
        + sensors.keys().filter(|(_, y)| *y == TROW).count();

    let xs = coverage(&sensors, TROW);
    // remove overlaps and calculate covered positions, minus beacons & sensors
    let r: i32 = canonicalize(&xs)
        .iter()
        .map(|(f, t)| t - f + 1)
        .sum::<i32>()
        - taken as i32;

    println!("part1: {}", r);

    const SMAX: i32 = 4000000; // 4000000
    for y in 0..=SMAX {
        let xs = canonicalize(&coverage(&sensors, y));
        // if we have a gap then we found the beacon
        if xs.len() > 1 {
            println!("part2: {}", SMAX as u64 * (xs[0].1 as u64 + 1) + y as u64);
            break;
        }
    }
}
