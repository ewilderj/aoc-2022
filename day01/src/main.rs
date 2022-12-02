use itertools::sorted;

fn main() {
    let s: Vec<u32> = include_str!("../input.txt")
        .split("\n\n")
        .map(|cals| cals.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    println!("part1: {}", s.iter().max().unwrap());
    println!("part2: {}", sorted(s.iter()).rev().take(3).sum::<u32>());
}
