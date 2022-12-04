fn ctov(c: char) -> u32 {
    if c >= 'a' {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn main() {
    let i = include_str!("../input.txt");
    println!(
        "part1: {}",
        &i.lines()
            .map(|s| s.split_at(s.len() / 2))
            .map(|(a, b)| a.chars().filter(|&c| b.contains(c)).next().unwrap())
            .map(ctov)
            .sum::<u32>()
    );

    println!(
        "part2: {}",
        &i.lines()
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(|s| s[0]
                .chars()
                .filter(|&f| s[1].contains(f) && s[2].contains(f))
                .next()
                .unwrap())
            .map(ctov)
            .sum::<u32>()
    );
}
