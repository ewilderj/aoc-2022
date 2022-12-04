fn main() {
    let nums: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            s.split(&[',', '-'][..])
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    println!(
        "part1: {}",
        nums.iter()
            .filter(|v| (v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1]))
            .count()
    );

    println!(
        "part2: {:?}",
        nums.iter()
            .filter(|v| (v[2] <= v[1] && v[2] >= v[0]) || (v[0] >= v[2] && v[0] <= v[3]))
            .count()
    );
}
