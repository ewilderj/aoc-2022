use std::collections::HashSet;
use std::iter;

fn newt(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    let (dx, dy) = (hx - tx, hy - ty);

    if dx.abs() > 1 || dy.abs() > 1 {
        (tx + dx.signum(), ty + dy.signum())
    } else {
        (tx, ty)
    }
}

fn hd(c: char) -> (i32, i32) {
    match c {
        'R' => (1, 0),
        'L' => (-1, 0),
        'U' => (0, 1),
        _ => (0, -1), // down
    }
}

fn runit(prog: &Vec<(i32, i32)>, rlen: usize) -> usize {
    let mut tp: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut x: Vec<i32> = vec![0; rlen];
    let mut y: Vec<i32> = vec![0; rlen];

    for (dx, dy) in prog.iter() {
        (x[0], y[0]) = (x[0] + dx, y[0] + dy);
        for n in 0..rlen - 1 {
            (x[n + 1], y[n + 1]) = newt(x[n], y[n], x[n + 1], y[n + 1]);
        }
        tp.insert((x[rlen - 1], y[rlen - 1]));
    }
    tp.len()
}

fn main() {
    // "compile" program by expanding the repetitions
    let prog: Vec<(i32, i32)> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (c, t) = s.split_once(' ').unwrap();
            iter::repeat(hd(c.chars().next().unwrap())).take(t.parse::<usize>().unwrap())
        })
        .flatten()
        .collect();

    println!("part1: {}", runit(&prog, 2));
    println!("part2: {}", runit(&prog, 10));
}
