use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    FORWARD,
    BACKWARD,
}
const F: Option<Direction> = Some(Direction::FORWARD);
const B: Option<Direction> = Some(Direction::BACKWARD);

type TreeSet = HashSet<(usize, usize)>;

// given a starting point (x0,y0) move across or down and record
// trees that meet the visibility criteria. max is always i32::MAX
// for part1, which lets us use that to vary the comparator
fn viz(
    v: &Vec<Vec<i32>>,
    x0: i32,
    y0: i32,
    across: Option<Direction>,
    down: Option<Direction>,
    max: i32,
) -> TreeSet {
    let mut r: TreeSet = TreeSet::new();
    let (w, h) = (v[0].len(), v.len());

    // if out of bounds, return empty set
    if x0 < 0 || y0 < 0 || x0 >= w as i32 || y0 >= h as i32 {
        return r;
    }

    let (mut x, mut y) = (x0 as usize, y0 as usize);

    let (dx, tx) = match across {
        F => (1, w - 1),
        B => (-1, 0),
        _ => (0, usize::MAX),
    };
    let (dy, ty) = match down {
        F => (1, h - 1),
        B => (-1, 0),
        _ => (0, usize::MAX),
    };

    let mut tallest: i32 = -1;
    loop {
        if max == i32::MAX {
            // part 1
            if v[y][x] > tallest {
                r.insert((x, y));
                tallest = v[y][x];
            }
        } else {
            // part 2
            r.insert((x, y));
            if v[y][x] >= max {
                return r;
            }
        }

        if x == tx || y == ty {
            return r;
        }
        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
    }
}

fn main() {
    let l: Vec<Vec<i32>> = include_str!("../input.txt")
        .lines()
        .map(|v| v.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    // yes, this solution works for non-square grids too!
    let (w, h) = (l[0].len() as i32, l.len() as i32);

    // viz computes the set of visibile trees from any point
    // for part1, we examine edges, and take the union of
    // all edge-visible trees.
    let t: TreeSet = (0..h as i32)
        .map(|n| {
            vec![
                viz(&l, 0, n, F, None, i32::MAX),
                viz(&l, w - 1, n, B, None, i32::MAX),
                viz(&l, n, 0, None, F, i32::MAX),
                viz(&l, n, h - 1, None, B, i32::MAX),
            ]
            .iter()
            .cloned()
            .flatten()
            .collect::<TreeSet>()
        })
        .flatten()
        .collect();

    println!("part1: {}", t.len());

    // for part2, we visit each tree and record each tree
    // until we hit one taller or the same
    let r = (0..w)
        .map(|x| {
            (0..h)
                .map(|y| {
                    let m = l[y as usize][x as usize];
                    vec![
                        viz(&l, x + 1, y, F, None, m),
                        viz(&l, x - 1, y, B, None, m),
                        viz(&l, x, y + 1, None, F, m),
                        viz(&l, x, y - 1, None, B, m),
                    ]
                    .iter()
                    .map(|s| s.len())
                    .fold(1, |a, n| a * n)
                })
                .collect::<Vec<usize>>()
        })
        .flatten()
        .max()
        .unwrap();

    println!("part2: {:?}", r);
}
