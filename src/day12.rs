use aoc_runner_derive::aoc;
use std::collections::{BTreeMap, VecDeque};

fn has_doubled_small(path: &[&str]) -> bool {
    for node in path {
        if node.chars().all(|c| c.is_ascii_uppercase()) {
            continue;
        }
        if path.iter().filter(|x| *x == node).count() == 2 {
            return true;
        }
    }
    false
}

fn solve(input: &str, small_limit: usize) -> usize {
    let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in input.lines() {
        let mut i = line.split("-");
        let a = i.next().unwrap();
        let b = i.next().unwrap();
        edges.entry(a.to_string()).or_default().push(b.to_string());
        edges.entry(b.to_string()).or_default().push(a.to_string());
    }

    let mut path_count = 0;
    let mut q = VecDeque::new();
    q.push_back(vec!["start"]);
    while let Some(path) = q.pop_front() {
        if path.last() == Some(&"end") {
            path_count += 1;
            continue;
        }
        for neighbor in &edges[*path.last().unwrap()] {
            if *neighbor != "start"
                && (neighbor.chars().all(|c| c.is_ascii_uppercase())
                    || !path.contains(&neighbor.as_str())
                    || (small_limit > 1 && !has_doubled_small(&path)))
            {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                q.push_back(new_path);
            }
        }
    }
    path_count
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    solve(input, 1)
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    solve(input, 2)
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample12.txt");
    assert_eq!(part1(&input), 226);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample12.txt");
    assert_eq!(part2(&input), 3509);
}
