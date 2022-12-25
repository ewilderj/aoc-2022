use std::cmp::Ordering;
use std::fmt;
use std::ops;

// Note:
// The aim of my solution is to learn as much about Rust as I can.
// So I wrote a formal solution that does two passes of lexing and
// then parsing, before computing over the resulting AST (a Signal).
// Lexing is trivial and could be omitted entirely, but it gave me
// a chance to learn about fmt::Display
//
// One nice benefit of this approach is that I was able to define
// an ordering on the Signal type, which made the actual final
// computations very elegant and simple.

#[derive(Debug, PartialEq, Eq, Clone)]
enum Signal {
    I(i32),
    L(Vec<Signal>),
}

impl Signal {
    // this is the logic for part1:
    // 0 for equal, 1 for right order, -1 for wrong order
    fn compare(l: &Signal, r: &Signal) -> i32 {
        if let Signal::I(m) = l {
            if let Signal::I(n) = r {
                // both numbers:
                return (n - m).signum();
            } else {
                // r is a list, so promote l and try again
                let pl = Signal::L(vec![l.clone()]);
                return Self::compare(&pl, r);
            }
        } else {
            // l is a list, so promote r and try again
            if let Signal::I(_) = r {
                let pr = Signal::L(vec![r.clone()]);
                return Self::compare(l, &pr);
            }

            // both sides are lists, so compare items
            let mut i = 0;
            if let (Signal::L(a), Signal::L(b)) = (l, r) {
                let lr = (b.len() as i32 - a.len() as i32).signum();
                loop {
                    if i >= a.len() || i >= b.len() {
                        return lr;
                    }
                    let c = Self::compare(&a[i], &b[i]);
                    if c != 0 {
                        return c;
                    }
                    i += 1;
                }
            }
        }
        return 0;
    }
}


impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        // use the part1 semantics to impose an ordering
        match Self::compare(&self, &other) {
            1 => Ordering::Less,
            -1 => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// I did a lot of work for Token, simply because I wanted to be
// able to print tokens out for debugging. But I learned about the Deref
// and Display traits, and how you need to impl for a Vec
// as well as the underlying type.

#[derive(Debug, PartialEq, Eq)]
enum Token {
    LPar,
    RPar,
    Num(u32),
    Skip,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LPar => write!(f, "["),
            Token::RPar => write!(f, "]"),
            Token::Num(x) => write!(f, "{x}"),
            _ => write!(f, "?"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TokenList(pub Vec<Token>);

impl ops::Deref for TokenList {
    type Target = Vec<Token>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TokenList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter()
            .fold(Ok(()), |r, t| r.and_then(|_| write!(f, "{} ", t)))
    }
}

fn lex(s: &str) -> TokenList {
    TokenList {
        0: s.chars()
            .map(|c| match c {
                '[' => Token::LPar,
                ']' => Token::RPar,
                _ => {
                    if let Some(d) = c.to_digit(11) {
                        Token::Num(d)
                    } else {
                        Token::Skip
                    }
                }
            })
            .filter(|c| *c != Token::Skip)
            .collect::<Vec<Token>>(),
    }
}

fn parse_aux(ts: &[Token]) -> Result<(usize, Signal), &str> {
    let mut i: usize = 0;
    let mut r: Vec<Signal> = Vec::new();

    while i < ts.len() {
        match ts[i] {
            Token::LPar => {
                let t = parse_aux(&ts[i + 1..]);
                if t.is_ok() {
                    let (ni, v) = t.unwrap();
                    r.push(v);
                    i += ni + 1;
                } else {
                    return t;
                }
            }
            Token::RPar => return Ok((i + 1, Signal::L(r))),
            Token::Num(n) => {
                r.push(Signal::I(n as i32));
                i += 1;
            }
            _ => {
                return Err("invalid token");
            }
        }
    }
    Ok((i + 1, Signal::L(r)))
}

fn parse(t: &TokenList) -> Result<Signal, &str> {
    let r = parse_aux(&t[1..t.len() - 1]);
    if r.is_ok() {
        let (_, v) = r.unwrap();
        return Ok(v);
    }
    return Err(r.err().unwrap());
}

fn marker(n: i32) -> Signal {
    Signal::L(vec![Signal::L(vec![Signal::I(n)])])
}

fn main() {
    let p1: usize = include_str!("../input.txt")
        .replace("10", "A") // convert numbers to base 11 makes lexing easier
        .split("\n\n")
        .map(|s| s.split_once('\n').unwrap())
        .map(|(x, y)| (lex(x), lex(y)))
        .map(|(x, y)| (parse(&x).unwrap(), parse(&y).unwrap()))
        .enumerate()
        .filter(|(_, (x, y))| x < y)
        .map(|(n, _)| n + 1)
        .sum();

    println!("part1: {:?}", p1);

    let mut l: Vec<_> = include_str!("../input.txt")
        .replace("10", "A")
        .lines()
        .filter(|l| l.len() > 0)
        .map(lex)
        .map(|x| parse(&x).unwrap())
        .collect();

    l.push(marker(2));
    l.push(marker(6));
    l.sort();

    let (m2, m6) = (marker(2), marker(6));

    let a = l.iter().position(|m| *m == m2).unwrap() + 1;
    let b = l.iter().position(|m| *m == m6).unwrap() + 1;

    println!("part2: {}", a * b);
}
