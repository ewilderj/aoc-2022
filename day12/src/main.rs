use pathfinding::prelude::astar;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(i32, i32);

impl Point {
    // viable neighbors and move cost (always 1)
    fn neighbors(&self, heights: &HashMap<Point, char>) -> Vec<(Point, usize)> {
        let &Point(x, y) = self;
        vec![
            Point(x + 1, y),
            Point(x - 1, y),
            Point(x, y + 1),
            Point(x, y - 1),
        ]
        .iter()
        .filter(|p| heights.contains_key(p) && heights[p] as i32 - heights[self] as i32 <= 1)
        .cloned()
        .map(|p| (p, 1))
        .collect()
    }

    // manhattan distance from another point
    fn distance(&self, other: &Point) -> usize {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
    }
}

const ORIGIN: Point = Point(0, 0);

fn main() {
    let mut start: Point = ORIGIN;
    let mut goal: Point = ORIGIN;
    let mut heights: HashMap<Point, char> = HashMap::new();
    // keep track of every 'a' point, for part2
    let mut a_ht: Vec<Point> = vec![];

    for (y, l) in include_str!("../input.txt").lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let p = Point(x as i32, y as i32);
            match c {
                'S' => {
                    start = p.clone();
                    a_ht.push(p.clone());
                    heights.insert(p, 'a');
                }
                'E' => {
                    goal = p.clone();
                    heights.insert(p, 'z');
                }
                _ => {
                    if c == 'a' {
                        a_ht.push(p.clone());
                    }
                    heights.insert(p, c);
                }
            }
        }
    }

    let r = astar(
        &start,
        |p| p.neighbors(&heights),
        |p| p.distance(&goal),
        |p| *p == goal,
    )
    .unwrap().1;

    println!("part1: {}", r);

    // now repeat the search, but for a starting point where a is the value
    let r = a_ht
        .iter()
        .map(|a| {
            astar(
                a,
                |p| p.neighbors(&heights),
                |p| p.distance(&goal),
                |p| *p == goal,
            )
        })
        .filter(|a| !a.is_none())
        .map(|a| a.unwrap().1)
        .sorted().next().unwrap();

    println!("part2: {}", r);
}
