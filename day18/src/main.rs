use pathfinding::prelude::astar;
use std::collections::HashSet;

type CubeSet = HashSet<(i32, i32, i32)>;

fn neighbors(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
    vec![
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y, z + 1),]
}

fn adjacent(d: &CubeSet, x: i32, y: i32, z: i32) -> usize {
    // perm each axis -1 and +1 to find adjacencies
    neighbors(x, y, z).iter().map(|c| d.contains(c)).filter(|b| *b).count()
}

fn neighborsa(d: &CubeSet, u: &CubeSet, x: i32, y: i32, z: i32) -> Vec<((i32, i32, i32), i32)> {
    neighbors(x, y, z).iter()
        // avoid moving to a rock, or a known unreachable
        .filter(|c| !d.contains(c) && !u.contains(c))
        .cloned()
        .map(|i| (i, 1))
        .collect()
}

fn unreachable(d: &CubeSet, c: &mut CubeSet, x: i32, y: i32, z: i32) -> bool {
    // air pocket if it's not lava and we have no access from the outside
    if !d.contains(&(x, y, z)) {
        let r = astar(
            &(x, y, z),
            |p: &(i32, i32, i32)| neighborsa(d, c, p.0, p.1, p.2),
            |p| ((x - p.0) + (y - p.1) + (z - p.2)).abs(),
            |p| *p == (0, 0, 0),
        );
        if r.is_none() {
            // cache known unreachable
            c.insert((x, y, z));
            return true;
        }
    }
    false
}

fn exterior(d: &CubeSet, x: i32, y: i32, z: i32, mx: i32, my: i32, mz: i32) -> bool {
    // true if there's line of sight on one axis to the min or max
    (0..x).map(|x0| !d.contains(&(x0, y, z))).all(|b| b)
        || (x + 1..=mx).map(|x0| !d.contains(&(x0, y, z))).all(|b| b)
        || (0..y).map(|y0| !d.contains(&(x, y0, z))).all(|b| b)
        || (y + 1..=my).map(|y0| !d.contains(&(x, y0, z))).all(|b| b)
        || (0..z).map(|z0| !d.contains(&(x, y, z0))).all(|b| b)
        || (z + 1..mz).map(|z0| !d.contains(&(x, y, z0))).all(|b| b)
}

fn main() {
    let d: CubeSet = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let mut x = l.split(",").map(|x| x.parse::<i32>().unwrap());
            (x.next().unwrap(), x.next().unwrap(), x.next().unwrap())
        })
        .collect();

    let p1: usize = d.len() * 6
        - d.iter()
            .map(|(x, y, z)| adjacent(&d, *x, *y, *z))
            .sum::<usize>();

    println!("part1: {}", p1);

    // find the maximum bounds of our puzzle grid
    let mx: i32 = d.iter().map(|t| t.0).max().unwrap();
    let my: i32 = d.iter().map(|t| t.1).max().unwrap();
    let mz: i32 = d.iter().map(|t| t.2).max().unwrap();

    let mut cache: CubeSet = HashSet::new();

    // approach: for non-exterior points, do an astar search to find
    // our way out: if we can't, then it's a pocket.
    //
    // could also have done this with dynamic programming and a
    // cache but this was mentally more straightforward :)
    for x in 1..=mx {
        for y in 1..=my {
            for z in 1..=mz {
                if !exterior(&d, x, y, z, mx, my, mz) {
                    _ = unreachable(&d, &mut cache, x, y, z);
                }
            }
        }
    }

    // interior surface area is the number of rock-facing sides
    // the air pockets have, so subtract that from our first answer
    let p2 = p1 - cache
        .iter()
        .map(|(x, y, z)| adjacent(&d, *x, *y, *z))
        .sum::<usize>();
    println!("part2: {p2}");
}
