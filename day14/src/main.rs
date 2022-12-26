use std::collections::HashMap;

type Strata = HashMap<(i32, i32), char>;

fn draw(m: &mut Strata, c: char, (x1, y1): (i32, i32), (x2, y2): (i32, i32)) {
    let mut x: i32 = x1;
    let mut y: i32 = y1;
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();

    // naive drawing that assumes perpendicular motion only
    while x != x2 || y != y2 {
        m.insert((x, y), c);
        x += dx;
        y += dy;
    }
    m.insert((x, y), c);
}

fn limits(m: &Strata) -> (i32, i32, i32, i32) {
    let xs: Vec<i32> = m.keys().map(|(x, _)| *x).collect();
    let ys: Vec<i32> = m.keys().map(|(_, y)| *y).collect();
    let (x1, x2) = (xs.iter().min().unwrap(), xs.iter().max().unwrap());
    let (y1, y2) = (&0, ys.iter().max().unwrap());
    (*x1, *y1, *x2, *y2)
}

fn output(m: &mut Strata) {
    let (x1, y1, x2, y2) = limits(m);

    for y in y1..=y2 {
        for x in x1..=x2 {
            print!("{}", m.get(&(x, y)).unwrap_or(&'.'));
        }
        println!("");
    }
}


fn main() {
    let l = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    let mut p = c.split(",");
                    (
                        p.next().unwrap().parse::<i32>().unwrap(),
                        p.next().unwrap().parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<_>>();

    let mut m: Strata = HashMap::from([((500, 0), '+')]);
    for ps in l.iter() {
        for i in 1..ps.len() {
            draw(&mut m, '#', ps[i - 1], ps[i]);
        }
    }

    let (minx, _, maxx, maxy) = limits(&m);

    let mut grains = 0;

    'outer: loop {
        let (mut sx, mut sy): (i32, i32) = (500, 0);
        loop {
            let oy = sy;
            for x in [sx, sx - 1, sx + 1] {
                if !m.contains_key(&(x, sy + 1)) {
                    sx = x;
                    sy += 1;
                    break;
                }
            }
            if oy == sy {
                // come to rest
                m.insert((sx, sy), 'o');
                grains += 1;
                break;
            }
            if sx < minx || sx > maxx || sy > maxy {
                // hit limits
                m.insert((sx, sy), 'X');
                break 'outer;
            }
        }
    }

    output(&mut m);
    println!("part1: {grains}");

    // get a clean scan to start over for part 2
    let mut m: Strata = HashMap::from([((500, 0), '+')]);
    for ps in l.iter() {
        for i in 1..ps.len() {
            draw(&mut m, '#', ps[i - 1], ps[i]);
        }
    }

    let mut grains = 0;

    'outer: loop {
        let (mut sx, mut sy): (i32, i32) = (500, 0);
        loop {
            let oy = sy;
            for x in [sx, sx - 1, sx + 1] {
                if !m.contains_key(&(x, sy + 1)) && sy < maxy + 1 {
                    sx = x;
                    sy += 1;
                    break;
                }
            }
            if oy == sy {
                // come to rest
                m.insert((sx, sy), 'o');
                grains += 1;
                if oy == 0 {
                    break 'outer;
                }
                break;
            }
        }
    }

    output(&mut m);
    println!("part2: {grains}");
}
