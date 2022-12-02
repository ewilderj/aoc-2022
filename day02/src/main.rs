const U_A: u32 = 'A' as u32;
const U_Z: u32 = 'X' as u32;

fn tuples(s: &str) -> (u32, u32) {
    let c: Vec<char> = s.chars().collect();
    return (c[0] as u32 - U_A, c[2] as u32 - U_Z);
}

fn score(t: (u32, u32)) -> u32 {
    match (2 + t.0 - t.1) % 3 {
        2 => t.1 + 4,
        1 => t.1 + 7,
        _ => t.1 + 1,
    }
}

fn new_t(t: (u32, u32)) -> (u32, u32) {
    (
        t.0,
        match t.1 {
            0 => (t.0 + 2) % 3,
            2 => (t.0 + 1) % 3,
            _ => t.0
        },
    )
}

fn main() {
    let i: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| tuples(l))
        .collect();

    println!("part1: {}", i.iter().map(|t| score(*t)).sum::<u32>());
    println!("part2: {}", i.iter().map(|t| score(new_t(*t))).sum::<u32>());
}
