use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Element {
    Pair(Box<Element>, Box<Element>),
    Number(u32),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Pair(l, r) => write!(f, "[{},{}]", l, r),
            Element::Number(v) => write!(f, "{}", v),
        }
    }
}

impl Element {
    fn new_pair(a: Element, b: Element) -> Self {
        Element::Pair(Box::new(a), Box::new(b))
    }

    fn num(&self) -> u32 {
        if let Self::Number(v) = self {
            *v
        } else {
            panic!("Attempted to get a regular number from a pair: {:?}", self);
        }
    }

    fn left_num_mut(&mut self) -> &mut u32 {
        match self {
            Element::Pair(l, _) => l.left_num_mut(),
            Element::Number(v) => v,
        }
    }

    fn right_num_mut(&mut self) -> &mut u32 {
        match self {
            Element::Pair(_, r) => r.right_num_mut(),
            Element::Number(v) => v,
        }
    }

    fn explode(&mut self) -> bool {
        self.explode_helper(0, None, None)
    }

    fn explode_helper(
        &mut self,
        depth: usize,
        left: Option<&mut Self>,
        right: Option<&mut Self>,
    ) -> bool {
        match self {
            Element::Pair(a, b) => {
                if depth == 4 {
                    if let Some(left) = left {
                        *left.right_num_mut() += a.num();
                    }
                    if let Some(right) = right {
                        *right.left_num_mut() += b.num();
                    }
                    *self = Element::Number(0);
                    return true;
                }
                if a.explode_helper(depth + 1, left, Some(b)) {
                    true
                } else {
                    b.explode_helper(depth + 1, Some(a), right)
                }
            }
            Element::Number(_) => false,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Element::Pair(a, b) => {
                if a.split() {
                    true
                } else {
                    b.split()
                }
            }
            Element::Number(v) if *v >= 10 => {
                *self = Element::new_pair(Element::Number(*v / 2), Element::Number((*v + 1) / 2));
                true
            }
            _ => false,
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Element::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
            Element::Number(v) => *v as usize,
        }
    }
}

impl std::ops::Add for Element {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        // println!("Adding {} and {}", self, rhs);
        let mut sum = Element::new_pair(self, rhs);
        loop {
            // println!("{}", &sum);
            if sum.explode() {
                continue;
            }
            if sum.split() {
                continue;
            }
            return sum;
        }
    }
}

impl Element {
    fn parse(input: &str) -> (&str, Self) {
        if &input[0..1] == "[" {
            let (input, a) = Self::parse(&input[1..]);
            assert_eq!(&input[0..1], ",");
            let (input, b) = Self::parse(&input[1..]);
            assert_eq!(&input[0..1], "]");
            (&input[1..], Element::new_pair(a, b))
        } else {
            (&input[1..], Element::Number(input[0..1].parse().unwrap()))
        }
    }
}

impl FromStr for Element {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (remaining, e) = Element::parse(s);
        if remaining.len() == 0 {
            Ok(e)
        } else {
            Err(())
        }
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Element> {
    input.lines().flat_map(|line| line.parse()).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Element]) -> usize {
    input
        .iter()
        .cloned()
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude()
}

#[aoc(day18, part2)]
fn part2(input: &[Element]) -> usize {
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                max = usize::max(max, (input[i].clone() + input[j].clone()).magnitude());
            }
        }
    } 
    max
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample18.txt");
    assert_eq!(part1(&parse(&input)), 3756);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample18.txt");
    assert_eq!(part2(&parse(&input)), 3993);
}
