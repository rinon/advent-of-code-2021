use aoc_runner_derive::aoc;
use std::collections::{BTreeSet, VecDeque};

fn neighbors(index: usize, width: usize, height: usize) -> Vec<usize> {
    let mut n = vec![];
    if index >= width {
        n.push(index - width);
    }
    if index < width * (height - 1) {
        n.push(index + width);
    }
    if index % width != 0 {
        n.push(index - 1);
    }
    if (index + 1) % width != 0 {
        n.push(index + 1);
    }
    n
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let width = input.split("\n").next().unwrap().len();
    let height = input.lines().count();

    let input: Vec<_> = input.chars().filter_map(|x| x.to_digit(10)).collect();

    let mut risk = 0usize;
    for (i, x) in input.iter().enumerate() {
        if neighbors(i, width, height).iter().all(|neighbor| *x < input[*neighbor]) {
            risk += *x as usize + 1;
        }
    }

    risk 
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let width = input.split("\n").next().unwrap().len();
    let height = input.lines().count();

    let input: Vec<_> = input.chars().filter_map(|x| x.to_digit(10)).collect();

    let mut low_points = vec![];
    for (i, x) in input.iter().enumerate() {
        if neighbors(i, width, height).iter().all(|neighbor| *x < input[*neighbor]) {
            low_points.push(i);
        }
    }

    let mut basins = vec![];
    for low in low_points {
        let mut basin = BTreeSet::new();
        let mut q = VecDeque::new();
        q.push_back(low);
        while let Some(i) = q.pop_front() {
            if input[i] == 9 {
                continue;
            }
            basin.insert(i);
            let neighbors = neighbors(i, width, height);
            let lower_neighbors = neighbors.iter().filter(|n| !basin.contains(n) && input[i] < input[**n]);
            q.extend(lower_neighbors);
        }
        dbg!(&basin);
        basins.push(basin);
    }
    basins.sort_by_key(|basin| basin.len());

    let mut total = 0;
    basins.iter().skip(basins.len() - 3).map(|b| b.len()).product()
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample9.txt");
    assert_eq!(part1(&input), 15);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample9.txt");
    assert_eq!(part2(&input), 1134);
}
