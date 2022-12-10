use std::collections::HashSet;

fn detect(s: &str, n: usize) -> usize {
    for i in 0..(s.len() - n) {
        let h: HashSet<char> = s[i..i + n].chars().collect();
        if h.len() == n {
            return i + n;
        }
    }
    unreachable!();
}

fn main() {
    let s = include_str!("../input.txt").trim();
    println!("part1: {}", detect(&s, 4));
    println!("part2: {}", detect(&s, 14));
}
