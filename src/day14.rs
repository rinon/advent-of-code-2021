use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    let mut input = input.lines();
    let template = input.next().unwrap();
    input.next();
    let rules: HashMap<(char, char), char> = input
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let mut left = left.chars();
            let (a, b) = (left.next().unwrap(), left.next().unwrap());
            ((a, b), right.chars().next().unwrap())
        })
        .collect();
    let mut polymer: Vec<char> = template.chars().collect();
    for _ in 0..10 {
        let mut new_polymer: Vec<char> = polymer
            .windows(2)
            .flat_map(|pair| {
                if let Some(insert) = rules.get(&(pair[0], pair[1])) {
                    vec![pair[0], *insert]
                } else {
                    vec![pair[0]]
                }
            })
            .collect();
        new_polymer.push(*polymer.last().unwrap());
        polymer = new_polymer;
    }
    let mut counts: HashMap<char, usize> = HashMap::new();
    for element in polymer {
        *counts.entry(element).or_default() += 1;
    }
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    let mut input = input.lines();
    let template = input.next().unwrap();
    input.next();
    let rules: HashMap<(char, char), char> = input
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let mut left = left.chars();
            let (a, b) = (left.next().unwrap(), left.next().unwrap());
            ((a, b), right.chars().next().unwrap())
        })
        .collect();
    let polymer: Vec<char> = template.chars().collect();
    let mut counts: HashMap<(char, char), usize> = HashMap::new();
    for pair in polymer.windows(2) {
        *counts.entry((pair[0], pair[1])).or_default() += 1;
    }
    for _ in 0..40 {
        // dbg!(&counts);
        let mut new_counts: HashMap<(char, char), usize> = HashMap::new();
        for (key, count) in &counts {
            if let Some(i) = rules.get(key) {
                *new_counts.entry((key.0, *i)).or_default() += count;
                *new_counts.entry((*i, key.1)).or_default() += count;
            } else {
                *new_counts.entry(*key).or_default() += count;
            }
        }
        counts = new_counts;
    }
    // dbg!(&counts);
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for (pair, count) in counts {
        *char_counts.entry(pair.0).or_default() += count;
        *char_counts.entry(pair.1).or_default() += count;
    }
    *char_counts.get_mut(&polymer[0]).unwrap() += 1;
    *char_counts.get_mut(&polymer.last().unwrap()).unwrap() += 1;
    char_counts.values().max().unwrap()/2 - char_counts.values().min().unwrap()/2
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample14.txt");
    assert_eq!(part1(&input), 1588);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample14.txt");
    assert_eq!(part2(&input), 2188189693529);
}
