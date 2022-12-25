use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Signal {
    I(i32),
    L(Vec<Signal>),
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    LPar,
    RPar,
    Num(u32),
    Skip,
}

// I did a lot of work simply because I wanted to be
// able to print tokens out. But I learned about the Deref
// and Display traits, and how you need to impl for a Vec
// as well as the underlying type.
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

fn compare(l: &Signal, r: &Signal) -> i32 {
    // println!("Compare {:?}\n        {:?}", l, r);

    if let Signal::I(m) = l {
        if let Signal::I(n) = r {
            // both numbers: 1 for right order, -1 for wrong. 0 for equal
            return (n - m).signum();
        } else {
            // r is a list, so promote l and try again
            let pl = Signal::L(vec![l.clone()]);
            return compare(&pl, r);
        }
    } else {
        // l is a list, so promote r and try again
        if let Signal::I(_) = r {
            let pr = Signal::L(vec![r.clone()]);
            return compare(l, &pr);
        }

        // both sides are lists, so compare items
        let mut i = 0;
        if let (Signal::L(a), Signal::L(b)) = (l, r) {
            let lr = (b.len() as i32 - a.len() as i32).signum();
            loop {
                if i >= a.len() || i >= b.len() {
                    return lr;
                }
                let c = compare(&a[i], &b[i]);
                if c != 0 {
                    return c;
                }
                i += 1;
            }
        }
    }
    return 0;
}

fn main() {
    // convert numbers to base 11 makes lexing easier: one char per token
    let p1: usize = include_str!("../input.txt")
        .replace("10", "A")
        .split("\n\n")
        .map(|s| s.split_once('\n').unwrap())
        .map(|(x, y)| (lex(x), lex(y)))
        .map(|(x, y)| (parse(&x).unwrap(), parse(&y).unwrap()))
        .enumerate()
        .filter(|(_, (x, y))| compare(&x, &y) == 1)
        .map(|(n, _)| n + 1)
        .sum();

    println!("part1: {:?}", p1);
}

// 5105 is too low
