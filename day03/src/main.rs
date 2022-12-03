fn ctov(c: char) -> u32 {
    if c >= 'a' {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn p1(s: &str) -> u32 {
    let c: Vec<char> = s.chars().collect();
    let mut i = c.chunks_exact(s.len() / 2);
    let (a, b) = (i.next().unwrap(), i.next().unwrap());
    for d in a {
        if b.contains(d) {
            return ctov(*d);
        }
    }
    unreachable!();
}

fn p2(s: &[&str]) -> u32 {
    for c in s[0].chars() {
        if s[1].contains(c) && s[2].contains(c) {
            return ctov(c);
        }
    }
    unreachable!();
}

fn main() {
    println!(
        "part1: {}",
        include_str!("../input.txt").lines().map(p1).sum::<u32>()
    );

    let m: Vec<_> = include_str!("../input.txt").lines().collect();
    println!("part2: {}", m.chunks_exact(3).map(p2).sum::<u32>());
}
