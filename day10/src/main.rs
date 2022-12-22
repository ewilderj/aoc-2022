fn main() {
    let l = include_str!("../input.txt")
        .lines()
        .map(|l| {
            if l.starts_with('n') {
                vec![0]
            } else {
                vec![0, l[5..].parse::<i32>().unwrap()]
            }
        })
        .flatten()
        .collect::<Vec<i32>>();

    let p1: i32 = [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&c| c as i32 * (1 + l.iter().take(c - 1).sum::<i32>()))
        .sum();
    println!("part1: {}", p1);

    let mut x: i32 = 1;
    let mut s = String::new();

    for (clock, n) in l.iter().enumerate() {
        if clock  % 40 == 0 {
            s.push('\n');
        }
        if ((clock % 40) as i32 - x).abs() <= 1 {
            s.push('#');
        } else {
            s.push('.');
        }
        x += n;
    }
    println!("part2:{s}");

    // ###..####.####.#..#.####.####.#..#..##..
    // #..#....#.#....#.#..#....#....#..#.#..#.
    // #..#...#..###..##...###..###..####.#..#.
    // ###...#...#....#.#..#....#....#..#.####.
    // #.#..#....#....#.#..#....#....#..#.#..#.
    // #..#.####.####.#..#.####.#....#..#.#..#.
}
