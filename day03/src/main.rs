fn ctov(c: char) -> u32 {
    if c >= 'a' {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn p1(s: &str) -> u32 {
    let m = s.len() / 2;
    let (a, b) = (&s[0..m], &s[m..]);

    for d in a.chars() {
        if b.contains(d) {
            return ctov(d);
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
    let i = include_str!("../input.txt");
    println!("part1: {}", &i.lines().map(p1).sum::<u32>());

    println!(
        "part2: {}",
        &i.lines()
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(p2)
            .sum::<u32>()
    );
}
