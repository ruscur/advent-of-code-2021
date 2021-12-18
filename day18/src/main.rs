use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Element {
    Value(usize),
    Pair(Vec<Element>),
}

#[derive(Debug)]
struct Explosion {
    x: usize,
    y: usize,
    left: Option<usize>,
    right: Option<usize>,
}

impl Element {
    fn get_left(&self) -> usize {
        match self {
            Element::Value(v) => *v,
            Element::Pair(p) => p[0].get_left(),
        }
    }

    fn add_left(&mut self, val: usize) {
        match self {
            Element::Value(v) => *self = Element::Value(*v + val),
            Element::Pair(p) => p[0].add_left(val),
        }
    }

    fn get_right(&self) -> usize {
        match self {
            Element::Value(v) => *v,
            Element::Pair(p) => p[1].get_right(),
        }
    }

    fn add_right(&mut self, val: usize) {
        match self {
            Element::Value(v) => *self = Element::Value(*v + val),
            Element::Pair(p) => p[1].add_right(val),
        }
    }

    fn explode(&mut self, depth: usize) -> Option<Explosion> {
        match self {
            Element::Value(_) => None,
            Element::Pair(p) => {
                if depth == 4 {
                    let x = self.get_left();
                    let y = self.get_right();

                    *self = Element::Value(0);

                    Some(Explosion {
                        x,
                        y,
                        left: None,
                        right: None,
                    })
                } else {
                    if let Some(mut e) = p[0].explode(depth + 1) {
                        if e.right.is_none() {
                            e.right = Some(p[1].get_left());
                            p[1].add_left(e.y);
                        }
                        Some(e)
                    } else if let Some(mut e) = p[1].explode(depth + 1) {
                        if e.left.is_none() {
                            e.left = Some(p[0].get_right());
                            p[0].add_right(e.x);
                        }
                        Some(e)
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Element::Value(v) => {
                if *v >= 10 {
                    let x = *v / 2;
                    let y = if x * 2 == *v { x } else { x + 1 };
                    *self = Element::Pair(vec![Element::Value(x), Element::Value(y)]);
                    true
                } else {
                    false
                }
            }
            Element::Pair(p) => {
                if p[0].split() {
                    return true;
                }
                p[1].split()
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Element::Value(v) => *v,
            Element::Pair(p) => 3 * p[0].magnitude() + 2 * p[1].magnitude(),
        }
    }
}

#[derive(Debug, Clone)]
struct Pair {
    x: Element,
    y: Element,
}

impl Pair {
    fn new(line: &Vec<char>) -> Pair {
        let mut opens = 0;
        let mut read = vec![];
        let mut left = vec![];

        // First pass
        for &c in line.iter().skip(1) {
            read.push(c);
            if c == '[' {
                opens += 1;
            } else if c == ']' {
                opens -= 1;
            } else if c == ',' && opens == 0 {
                read.pop();
                left = read;
                read = vec![];
            }
        }
        read.pop();
        let right = read;

        let x = if left.len() == 1 {
            Element::Value(left[0].to_digit(10).unwrap() as usize)
        } else {
            let pair = Pair::new(&left);
            Element::Pair(vec![pair.x, pair.y])
        };

        let y = if right.len() == 1 {
            Element::Value(right[0].to_digit(10).unwrap() as usize)
        } else {
            let pair = Pair::new(&right);
            Element::Pair(vec![pair.x, pair.y])
        };

        Pair { x, y }
    }

    fn explode(&mut self) -> bool {
        if let Some(e) = self.x.explode(1) {
            if e.right.is_none() {
                self.y.add_left(e.y);
            }
            true
        } else if let Some(e) = self.y.explode(1) {
            if e.left.is_none() {
                self.x.add_right(e.x);
            }
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        self.x.split() || self.y.split()
    }

    fn to_element(&self) -> Element {
        Element::Pair(vec![self.x.clone(), self.y.clone()])
    }

    fn add(&self, other: &Pair) -> Pair {
        let mut pair = Pair {
            x: self.to_element(),
            y: other.to_element(),
        };

        while pair.explode() || pair.split() {}

        pair
    }

    fn magnitude(&self) -> usize {
        3 * self.x.magnitude() + 2 * self.y.magnitude()
    }
}

fn parse(lines: &Vec<String>) -> Result<Vec<Pair>, Box<dyn std::error::Error>> {
    let pairs = lines
        .iter()
        .map(|l| Pair::new(&l.chars().collect()))
        .collect();

    Ok(pairs)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let pairs = parse(&lines?)?;

    let result = pairs
        .clone()
        .into_iter()
        .reduce(|prev, next| prev.add(&next))
        .unwrap();

    println!("Part 1: {}", result.magnitude());

    let max = pairs
        .clone()
        .into_iter()
        .filter_map(|p1| {
            pairs
                .clone()
                .into_iter()
                .map(|p2| p1.add(&p2).magnitude())
                .max()
        })
        .max()
        .unwrap();

    println!("Part 2: {}", max);

    Ok(())
}
