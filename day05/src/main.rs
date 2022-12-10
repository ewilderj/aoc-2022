fn main() {
    let l = include_str!("../input.txt");
    let n = (l.find('\n').unwrap() + 1) / 4;

    // make some empty towers
    let mut t: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        t.push(Vec::new());
    }

    // set up the initial configuration
    let (ts, is) = l.split_once("\n\n").unwrap();
    for s in ts.lines().map(|s| s.chars().collect::<Vec<char>>()) {
        for i in 0..n {
            let m = i * 4 + 1;
            if s[m].is_ascii_alphabetic() {
                t[i].insert(0, s[m]);
            }
        }
    }

    // create our program
    let prog: Vec<_> = is
        .lines()
        .map(|m| {
            let p: Vec<&str> = m.split_ascii_whitespace().collect();
            (
                p[1].parse::<usize>().unwrap(),
                p[3].parse::<usize>().unwrap() - 1,
                p[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    // execute on the CrateMover 9000
    let mut t1 = t.clone();
    for (num, from, to) in prog.iter() {
        for _ in 0..*num {
            let r = t1[*from].pop().unwrap();
            t1[*to].push(r);
        }
    }
    let p1: String = t1.iter().map(|v| v.get(v.len() - 1).unwrap()).collect();
    println!("part1: {}", p1);

    // execute on the CrateMover 9001
    let mut t2 = t.clone();
    for (num, from, to) in prog.iter() {
        let pos = t2[*from].len() - *num;
        let mut tt: Vec<char> = t2[*from].split_off(pos);
        t2[*to].append(&mut tt);
    }
    let p2: String = t2.iter().map(|v| v.get(v.len() - 1).unwrap()).collect();
    println!("part2: {}", p2);
}
